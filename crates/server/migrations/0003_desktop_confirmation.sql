-- Desktop credentials become eligible for future ownership only after the
-- native client has persisted them. Existing bindings must confirm again.
ALTER TABLE account_sessions ADD COLUMN desktop_confirmed_at timestamptz;
