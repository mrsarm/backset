CREATE TABLE IF NOT EXISTS elements (
    tid         VARCHAR(40) NOT NULL,
    id          VARCHAR(40) NOT NULL,
    data        JSONB NOT NULL,
    created_at  TIMESTAMP NOT NULL,

    CONSTRAINT elements_pkey PRIMARY KEY (tid, id),
    CONSTRAINT elements_tid_fkey FOREIGN KEY(tid) REFERENCES tenants(id)
);

CREATE INDEX IF NOT EXISTS elements_tid_created_at_idx ON elements (tid, created_at DESC);
