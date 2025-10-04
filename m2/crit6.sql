

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