create table if not exists districts (
    district_id         TEXT                  PRIMARY KEY NOT NULL,
    name                VARCHAR(255)          UNIQUE NOT NULL,
    link                VARCHAR(255)          NOT NULL,
    created_at          TIMESTAMP             NOT NULL
)