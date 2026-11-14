-- Keep match-probe timing separate from heartbeat/live-upload activity.
ALTER TABLE recordings ADD COLUMN probe_observed_at timestamptz;

-- Recover the receive time of the chunk that supplied the stored probe.
-- Without such a chunk the probe still comes from session creation.
UPDATE recordings r
SET probe_observed_at = coalesce(
    (SELECT min(c.received_at)
     FROM recording_chunks c
     WHERE c.recording_id = r.id AND c.last_game_time = r.last_game_time),
    r.created_at
);

ALTER TABLE recordings
    ALTER COLUMN probe_observed_at SET DEFAULT now(),
    ALTER COLUMN probe_observed_at SET NOT NULL;
