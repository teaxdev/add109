-- This SQL script includes intentional syntax errors -- find and correct them, then run the result 

-- 1.find all Zelda games playable on some version of the Game Boy system
SELECT * FROM Zelda WHERE Systems LIKE '%Game Boy%';

-- 2.update all records having the Timeline "Child Timeline" to read "Child timeline"
UPDATE Zelda SET Timeline = 'Child timeline';

-- 3.add a new record
INSERT INTO Zelda (Timeline, Title, ReleaseYear, Systems)
VALUES ('Imaginary timeline', 'The Future of Zelda', 2034, 'Super Duper New Nintendo Switcheroo');

-- 4.remove the record with GameID 3
DELETE FROM Zelda WHERE GameID = 3;

-- 5. Display the corrected results (this statement has correct syntax)
SELECT * FROM Zelda;