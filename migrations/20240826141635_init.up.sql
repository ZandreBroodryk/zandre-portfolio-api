-- Add up migration script here
CREATE TABLE
  IF NOT EXISTS users (
    id serial PRIMARY KEY,
    created_at timestamp
    with
      time zone DEFAULT "now" () NOT NULL,
      email varchar(256) NOT NULL UNIQUE,
      password_hash text NOT NULL,
      display_name varchar(256) NOT NULL
  );

CREATE TABLE
  IF NOT EXISTS blog_posts (
    id serial PRIMARY KEY,
    created_at timestamp without time zone DEFAULT "now" () NOT NULL,
    title varchar(256) NOT NULL,
    content text NOT NULL,
    author int NOT NULL REFERENCES users
  );