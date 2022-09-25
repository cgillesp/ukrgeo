-- Your SQL goes here
CREATE TABLE geolocations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT,
    lat DOUBLE,
    lon DOUBLE,
    url TEXT
)