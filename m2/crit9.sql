DROP TABLE IF EXISTS players;
DROP VIEW IF EXISTS top_players;

CREATE TABLE players (id INTEGER PRIMARY KEY, username TEXT, wins INTEGER, matches_played INTEGER);

INSERT INTO players (username, wins, matches_played) VALUES 
  ('alex', 12, 20),
  ('blaze', 7, 15),
  ('cyra', 0, 3),
  ('drew', 25, 30),
  ('echo', 9, 12),
  ('frost', 14, 22),
  ('gryphon', 3, 8),
  ('nova', 18, 27),
  ('raven', 6, 11),
  ('zeno', 21, 29);
  
CREATE VIEW top_players AS 
SELECT * FROM players
WHERE wins >= (SELECT AVG(wins) FROM players)
ORDER BY wins DESC;

SELECT * FROM top_players;

WITH struggling_players AS (
  SELECT * FROM players WHERE wins < (SELECT AVG(wins) FROM players)
)
SELECT username, wins, matches_played FROM struggling_players;