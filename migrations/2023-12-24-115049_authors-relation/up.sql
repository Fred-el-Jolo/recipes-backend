-- Your SQL goes here
PRAGMA foreign_keys=off;

ALTER TABLE tweets RENAME TO tweets_old;

CREATE TABLE tweets
(
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  message TEXT NOT NULL,
  author_id INTEGER,
  CONSTRAINT fk_author
    FOREIGN KEY (author_id)
    REFERENCES user (id)
);

INSERT INTO tweets (id, created_at, message) SELECT * FROM tweets_old;

DROP TABLE tweets_old;

PRAGMA foreign_keys=on;
