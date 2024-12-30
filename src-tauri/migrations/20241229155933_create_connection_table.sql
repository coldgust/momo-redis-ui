CREATE TABLE IF NOT EXISTS connection
(
    id                         INTEGER PRIMARY KEY AUTOINCREMENT,
    name                       VARCHAR(1024) NOT NULL,
    username                   VARCHAR(1024),
    password                   VARCHAR(1024),
    host                       VARCHAR(1024),
    port                       INTEGER,
    sentinel_node_password     VARCHAR(1024),
    sentinel_master_group_name VARCHAR(1024),
    type                       VARCHAR(50),
    read_only                  INTEGER,
    separator                  VARCHAR(50),
    cert_path                  TEXT,
    key_path                   TEXT,
    ca_path                    TEXT,
    hostname_verify            INTEGER
);