-- Your SQL goes here
CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  login TEXT NOT NULL,
  name TEXT NOT NULL,
  password TEXT NOT NULL
);

INSERT INTO USERS VALUES(0, 'admin', 'admin', '@dmin');
