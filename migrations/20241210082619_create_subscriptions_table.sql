-- migration/{timestamp}_create_subscriptions_table.sql
CREATE TABLE subscriptions (
    id uuid NOT NULL,
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (email)
);
