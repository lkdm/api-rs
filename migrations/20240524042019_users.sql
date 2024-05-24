-- Add migration script here
CREATE TABLE Users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT(255) NOT NULL
);
