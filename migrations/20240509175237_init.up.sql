-- Add up migration script here
CREATE TABLE company (
    id INTEGER NOT NULL CONSTRAINT PK_company PRIMARY KEY,
    name TEXT NOT NULL,
    remainder_begin_month REAL NOT NULL,
    debit_turnover REAL NOT NULL,
    credit_turnover REAL NOT NULL,
    remainder_end_month REAL NOT NULL
);


