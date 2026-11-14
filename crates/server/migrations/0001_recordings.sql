-- Recordings, chunks and share codes (spec §15.2). Never DELETE from these tables (D1).

CREATE TYPE end_reason          AS ENUM ('game_end_event', 'lcu_end_of_game', 'lcu_left_game', 'superseded', 'timeout');
CREATE TYPE game_result         AS ENUM ('win', 'lose', 'remake', 'unknown');
CREATE TYPE riot_game_source    AS ENUM ('lcu', 'spectator');
CREATE TYPE share_mode          AS ENUM ('live', 'public');
CREATE TYPE share_delete_reason AS ENUM ('game_ended', 'revoked');

CREATE TABLE devices (
    id            uuid PRIMARY KEY DEFAULT uuidv7(),
    secret_hash   bytea       NOT NULL CHECK (length(secret_hash) = 32),
    app_version   text        NOT NULL,
    created_at    timestamptz NOT NULL DEFAULT now(),
    last_seen_at  timestamptz NOT NULL DEFAULT now(),
    deleted_at    timestamptz
);

CREATE TABLE recordings (
    id                  uuid PRIMARY KEY DEFAULT uuidv7(),
    device_id           uuid        NOT NULL REFERENCES devices(id),

    -- match identity
    game_mode           text        NOT NULL,           -- GameMode (open enum), e.g. 'CLASSIC', 'KIWI'
    map_number          smallint    NOT NULL,
    fingerprint         bytea       NOT NULL CHECK (length(fingerprint) = 32),
    probe               bytea       NOT NULL,           -- bincode Probe of the latest snapshot
    platform_id         text,                           -- Platform (open enum), e.g. 'NA1'
    riot_game_id        bigint,
    riot_game_source    riot_game_source,
    queue_id            integer,
    is_custom           boolean,
    is_practice         boolean     NOT NULL DEFAULT false,

    -- who played
    active_riot_id      text        NOT NULL,
    active_puuid        text,
    champion            text        NOT NULL,           -- lowercase internal key (§8.6), e.g. 'monkeyking'

    -- timing (server clock)
    started_at_estimate timestamptz NOT NULL,
    first_game_time     real        NOT NULL,
    last_game_time      real        NOT NULL,
    coverage_s          integer     NOT NULL DEFAULT 0,
    last_ingest_at      timestamptz NOT NULL DEFAULT now(),
    ended_at            timestamptz,
    end_reason          end_reason,
    result              game_result,
    game_length_s       integer,

    -- summary from the latest snapshot
    kills               smallint    NOT NULL DEFAULT 0,
    deaths              smallint    NOT NULL DEFAULT 0,
    assists             smallint    NOT NULL DEFAULT 0,
    creep_score         smallint    NOT NULL DEFAULT 0,
    level               smallint    NOT NULL DEFAULT 1,

    app_version         text        NOT NULL,
    created_at          timestamptz NOT NULL DEFAULT now(),
    deleted_at          timestamptz,

    CHECK ((ended_at IS NULL) = (end_reason IS NULL))
);

CREATE INDEX recordings_by_device   ON recordings (device_id, id DESC);
CREATE INDEX recordings_open        ON recordings (last_ingest_at)
    WHERE ended_at IS NULL AND deleted_at IS NULL;
CREATE UNIQUE INDEX recordings_one_open_per_device ON recordings (device_id)
    WHERE ended_at IS NULL AND deleted_at IS NULL;
CREATE INDEX recordings_by_riot_game ON recordings (platform_id, riot_game_id)
    WHERE riot_game_id IS NOT NULL;            -- not unique: several devices may record one match
CREATE INDEX recordings_by_fingerprint ON recordings (fingerprint);

CREATE TABLE recording_chunks (
    recording_id      uuid        NOT NULL REFERENCES recordings(id),
    seq               integer     NOT NULL CHECK (seq >= 0),
    first_game_time   real        NOT NULL,
    last_game_time    real        NOT NULL,
    snapshot_count    smallint    NOT NULL CHECK (snapshot_count BETWEEN 1 AND 120),
    raw_format        smallint    NOT NULL,
    raw_sha256        bytea       NOT NULL CHECK (length(raw_sha256) = 32),
    raw_size          integer     NOT NULL,      -- uncompressed bincode bytes
    raw               bytea       NOT NULL,      -- brotli(RawChunk), as uploaded
    frames_format     smallint    NOT NULL,
    frames            bytea       NOT NULL,      -- brotli(FrameChunk), served with Content-Encoding: br
    received_at       timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (recording_id, seq)
);
-- Values are already compressed: keep them out of line without pglz/lz4 recompression.
ALTER TABLE recording_chunks ALTER COLUMN raw    SET STORAGE EXTERNAL;
ALTER TABLE recording_chunks ALTER COLUMN frames SET STORAGE EXTERNAL;

CREATE TABLE share_codes (
    id              uuid PRIMARY KEY DEFAULT uuidv7(),
    recording_id    uuid        NOT NULL REFERENCES recordings(id),
    code            text        NOT NULL UNIQUE CHECK (code ~ '^[0-9A-HJKMNP-TV-Z]{8}$'),
    mode            share_mode  NOT NULL,
    created_at      timestamptz NOT NULL DEFAULT now(),
    mode_changed_at timestamptz NOT NULL DEFAULT now(),
    deleted_at      timestamptz,
    delete_reason   share_delete_reason,
    CHECK ((deleted_at IS NULL) = (delete_reason IS NULL))
);
CREATE UNIQUE INDEX share_codes_one_active ON share_codes (recording_id) WHERE deleted_at IS NULL;

CREATE TABLE riot_id_cache (
    riot_id     text PRIMARY KEY,           -- 'gameName#tagLine'
    puuid       text NOT NULL,
    resolved_at timestamptz NOT NULL DEFAULT now()
);
