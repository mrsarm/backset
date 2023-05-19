CREATE TABLE tenants (
    -- id BIGSERIAL PRIMARY KEY,  --> `pub id: i64` in the Rust model
    id VARCHAR(40) NOT NULL UNIQUE,
    name VARCHAR(80) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL
);
