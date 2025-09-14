CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    birth_year INTEGER NOT NULL,
    books INTEGER[] NOT NULL DEFAULT ARRAY[]::INTEGER[],
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    author_id INTEGER NOT NULL,
    publisher_id INTEGER NOT NULL,
    isbn TEXT NOT NULL,
    year INTEGER NOT NULL,
    price REAL NOT NULL,
    quantity INTEGER NOT NULL,
    borrowed_by INTEGER[] NOT NULL DEFAULT ARRAY[]::INTEGER[],
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE publishers (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    books INTEGER[] NOT NULL DEFAULT ARRAY[]::INTEGER[],
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    membership_id TEXT NOT NULL,
    phone TEXT NOT NULL,
    subscription_days_left INTEGER NOT NULL,
    current_books INTEGER[] NOT NULL DEFAULT ARRAY[]::INTEGER[],
    borrowed_history INTEGER[] NOT NULL DEFAULT ARRAY[]::INTEGER[],
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);