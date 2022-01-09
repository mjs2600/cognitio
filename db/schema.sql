CREATE TABLE IF NOT EXISTS "schema_migrations" (version varchar(255) primary key);
CREATE TABLE books (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  isbn TEXT UNIQUE
);
CREATE UNIQUE INDEX idx_books_isbn ON books (isbn);
CREATE INDEX idx_books_title ON books (title);
-- Dbmate schema migrations
INSERT INTO "schema_migrations" (version) VALUES
  ('20220109202829');
