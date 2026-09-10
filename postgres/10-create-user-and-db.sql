-- file: 10-create-user-and-db.sql
CREATE DATABASE persons;
CREATE ROLE program WITH PASSWORD 'test';
GRANT ALL PRIVILEGES ON DATABASE persons TO program;
ALTER ROLE program WITH LOGIN;

\connect persons

CREATE TABLE IF NOT EXISTS persons
(
    id      SERIAL PRIMARY KEY,
    name    VARCHAR(255) NOT NULL,
    age     INT,
    address VARCHAR(255),
    work    VARCHAR(255)
);

ALTER TABLE persons OWNER TO program;
ALTER SEQUENCE persons_id_seq OWNER TO program;
