create table if not exists super_districts (
    super_district_id   INTEGER PRIMARY KEY   NOT NULL,
    name                VARCHAR(255)          UNIQUE NOT NULL,
    created_at          TIMESTAMP             DEFAULT CURRENT_TIMESTAMP NOT NULL
)