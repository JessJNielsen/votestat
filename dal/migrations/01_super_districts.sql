create table if not exists super_districts (
    super_district_id   TEXT                  PRIMARY KEY NOT NULL,
    name                VARCHAR(255)          UNIQUE NOT NULL,
    created_at          TIMESTAMP             NOT NULL
)