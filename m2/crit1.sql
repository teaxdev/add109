SELECT * FROM Country ORDER BY Name;

INSERT INTO Country (Name, Code, LifeExpectancy, Continent)
    VALUES ('Petoria', 'PTR', '1000', 'North America');
-- 169
UPDATE Country SET Name = 'French Republic';
-- 74
DELETE FROM Country WHERE Code = 'PTR';
SELECT * FROM Country;
