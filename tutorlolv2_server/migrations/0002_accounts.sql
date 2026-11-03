CREATE TYPE session_kind AS ENUM ('web','desktop');
CREATE TABLE accounts (
 id uuid PRIMARY KEY DEFAULT uuidv7(), google_sub text NOT NULL UNIQUE,
 email text NOT NULL, created_at timestamptz NOT NULL DEFAULT now(),
 last_login_at timestamptz NOT NULL DEFAULT now(), deleted_at timestamptz
);
CREATE TABLE account_sessions (
 id uuid PRIMARY KEY DEFAULT uuidv7(), account_id uuid NOT NULL REFERENCES accounts(id),
 token_hash bytea NOT NULL UNIQUE CHECK(length(token_hash)=32), kind session_kind NOT NULL,
 device_id uuid REFERENCES devices(id), created_at timestamptz NOT NULL DEFAULT now(),
 last_used_at timestamptz NOT NULL DEFAULT now(), expires_at timestamptz NOT NULL, revoked_at timestamptz,
 CHECK((kind='desktop')=(device_id IS NOT NULL))
);
CREATE INDEX account_sessions_device ON account_sessions(device_id) WHERE revoked_at IS NULL;
ALTER TABLE devices ADD COLUMN account_id uuid REFERENCES accounts(id);
ALTER TABLE devices ADD COLUMN account_session_id uuid REFERENCES account_sessions(id);
ALTER TABLE recordings ADD COLUMN account_id uuid REFERENCES accounts(id);
CREATE INDEX recordings_by_account ON recordings(account_id,id DESC) WHERE account_id IS NOT NULL;
