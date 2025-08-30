-- I will list countries by name, add 'Petoria', update France, and lastly delete Petoria.
SELECT * FROM Country ORDER BY Name;

INSERT INTO Country (Name, Code, LifeExpectancy, Continent)
    VALUES ('Petoria', 'PTR', '1000', 'North America');
-- 169
UPDATE Country SET Name = 'French Republic' WHERE Code = 'FRA';
-- 74
DELETE FROM Country WHERE Code = 'PTR';
