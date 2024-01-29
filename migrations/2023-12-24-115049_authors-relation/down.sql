-- This file should undo anything in `up.sql`

PRAGMA foreign_keys=off;

ALTER TABLE tweets RENAME TO tweets_old;

CREATE TABLE tweets
(
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  message TEXT NOT NULL
);

INSERT INTO tweets SELECT id, created_at, message FROM tweets_old;

DROP TABLE tweets_old;

COMMIT;

PRAGMA foreign_keys=on;
