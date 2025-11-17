CREATE TABLE messages
(
    id         SERIAL PRIMARY KEY,
    content    TEXT                      NOT NULL,
    title      TEXT,
    author     TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    ip         INET                      NOT NULL
);

CREATE TABLE prints
(
    id         SERIAL PRIMARY KEY,
    message    INT                       NOT NULL REFERENCES messages (id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);
