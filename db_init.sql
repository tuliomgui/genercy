-- Add migration script here
CREATE TABLE IF NOT EXISTS tb_user2 (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(250) NOT NULL
);