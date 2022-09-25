-- Your SQL goes here
ALTER TABLE geolocations ADD COLUMN uuid BLOB;
ALTER TABLE geolocations ADD COLUMN notes TEXT;
ALTER TABLE geolocations ADD COLUMN geolocator TEXT;
ALTER TABLE geolocations ADD COLUMN datetime DATETIME;
