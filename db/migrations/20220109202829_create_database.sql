-- migrate:up

CREATE TABLE books (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  isbn TEXT UNIQUE
);

CREATE UNIQUE INDEX idx_books_isbn ON books (isbn);
CREATE INDEX idx_books_title ON books (title);

-- migrate:down

DROP TABLE books;
