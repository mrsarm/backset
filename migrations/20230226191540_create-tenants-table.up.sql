CREATE TABLE IF NOT EXISTS tenants (
    -- id BIGSERIAL PRIMARY KEY,  --> `pub id: i64` in the Rust model
    id VARCHAR(40) PRIMARY KEY,
    name VARCHAR(80) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS tenants_created_at_idx ON tenants (created_at DESC);
