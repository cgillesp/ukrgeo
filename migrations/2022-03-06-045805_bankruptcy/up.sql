-- Your SQL goes here
DROP TABLE IF EXISTS geolocations;
CREATE TABLE geolocations (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    description TEXT,
    lat DOUBLE NOT NULL,
    lon DOUBLE NOT NULL,
    url TEXT NOT NULL,
    uuid BLOB NOT NULL,
    notes TEXT,
    geolocator TEXT,
    datetime DATETIME,
    provisional BOOLEAN
);
CREATE INDEX uuid_idx ON geolocations (uuid);