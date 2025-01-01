CREATE TABLE IF NOT EXISTS connection
(
    id                         INTEGER PRIMARY KEY AUTOINCREMENT,
    name                       VARCHAR(1024) NOT NULL,
    username                   VARCHAR(1024),
    password                   VARCHAR(1024),
    host                       VARCHAR(1024),
    port                       INTEGER,
    db                         INTEGER DEFAULT 0,
    sentinel_node_password     VARCHAR(1024),
    sentinel_master_group_name VARCHAR(1024),
    type                       VARCHAR(50),
    read_only                  INTEGER DEFAULT 0,
    separator                  VARCHAR(50),
    enable_ssl                 INTEGER DEFAULT 0,
    cert_path                  TEXT,
    key_path                   TEXT,
    ca_path                    TEXT,
    hostname_verify            INTEGER DEFAULT 0
);