-- Your SQL goes here

CREATE TABLE IF NOT EXISTS Events (
    Id UUID,
    Name TEXT NOT NULL,
    Venue TEXT NOT NULL,
    Address TEXT,
    Image VARCHAR(255),
    Comments TEXT,
    ContactName VARCHAR(255),
    Starts_at TIMESTAMP NOT NULL DEFAULT NOW(),
    Ends_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(Id)
)

