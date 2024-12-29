-- Add migration script here
CREATE TABLE IF NOT EXISTS connection
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    name     VARCHAR(255) NOT NULL,
    username VARCHAR(255),
    password VARCHAR(255),
    address  VARCHAR(255)
);