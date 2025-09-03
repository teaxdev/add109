DROP TABLE IF EXISTS car;
CREATE TABLE car (
    id INTEGER PRIMARY KEY,
    value NOT NULL CHECK (value >= 1000),
    license_plate UNIQUE,
    condition TEXT DEFAULT 'used',
    model INTEGER
);

INSERT INTO car (value, license_plate, condition, model)
VALUES 
    (8000, 'ABC123', 'used', 2020),
    (15000, 'XYZ789', 'new', 2023),
    (5000, 'DEF456', 'used', 2018);

SELECT DISTINCT condition FROM car;

ALTER TABLE car ADD COLUMN color TEXT DEFAULT 'white';

SELECT
    value,
    CASE
        WHEN value <= 10000 THEN 'This is a cheap car'
        WHEN value < 50000 THEN 'This is an average car'
        ELSE 'This is an expensive car'
    END AS car_price
FROM car;

SELECT * FROM car;

CREATE INDEX idx_license_plate ON car(license_plate);