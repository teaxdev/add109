DROP TABLE IF EXISTS people;
CREATE TABLE people (
    id INTEGER PRIMARY KEY,
    name,
    member_until DEFAULT (DATE('now', '+30 days')),
    time_zone DEFAULT (TIME('now', '-5 hours')),
    created DEFAULT (DATETIME('now'))
);

INSERT INTO people (name, member_until, time_zone, created)
VALUES 
  ('Alice',   '2025-11-05', '07:00:00', '2025-10-06 12:00:00'),
  ('Bob',     DATE('now', '+60 days'), '08:00:00', '2025-10-01 09:30:00'),
  ('Charlie', unixepoch('now', '+8640000 seconds'), '06:30:00', '2025-09-25 14:45:00'),
  ('Diana',   julianday('now', '+45 days'), '05:45:00', '2025-10-05 10:15:00'),
  ('Evan',    '2026-01-01', strftime('%T', 'now', '-8 hours'), '2025-10-06 08:00:00');

INSERT INTO people (name)
VALUES ('Max');

SELECT DATE('now', '+1 days');

SELECT TIME('now', '-8 hours');

SELECT DATETIME('now', '+3 days');

SELECT unixepoch('now');
SELECT unixepoch('now', '+1000000 seconds');

SELECT julianday('now');
SELECT julianday('now', '+10000 days');

SELECT strftime('%F', 'now', '+50 days');
SELECT strftime('%T', 'now', '+10000 minutes');
SELECT CAST(strftime('%s', 'now', '+1000000 seconds') as INT);

SELECT TIME('now', '-5 hours');

SELECT * FROM people;