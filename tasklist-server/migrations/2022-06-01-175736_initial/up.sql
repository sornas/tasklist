-- Your SQL goes here

CREATE TABLE routine (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    model INTEGER NOT NULL
);

CREATE TABLE model (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    routine INTEGER NOT NULL
);

CREATE TABLE task (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    state VARCHAR NOT NULL  -- NOTE: could be INTEGER for less storage
);

CREATE TABLE tasklist (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    state VARCHAR NOT NULL,  -- NOTE: could be INTEGER for less storage
    routine_id INTEGER NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE task_partof_regular (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    regular INTEGER NOT NULL,
    task INTEGER NOT NULL
);

CREATE TABLE task_partof_model (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    model INTEGER NOT NULL,
    task INTEGER NOT NULL
);

-- CREATE TABLE routines (
--     id INT PRIMARY KEY,
--     name VARCHAR NOT NULL,
--     -- repetition INT,
--     model INT NOT NULL,
--     owner INT NOT NULL,
-- );
-- 
-- CREATE TABLE models (
--     id INT PRIMARY KEY,
--     routine INT NOT NULL,
-- );
-- 
-- CREATE TABLE users (
--     id INT PRIMARY KEY,
--     name VARCHAR NOT NULL,
--     password VARCHAR NOT NULL,
--     email VARCHAR NOT NULL,
-- );
-- 
-- CREATE TABLE regular (
--     id INT PRIMARY KEY,
--     starttime DATETIME,
--     endtime DATETIME,
-- );
-- 
-- CREATE TABLE tasks (
--     id INT PRIMARY KEY,
--     name VARCHAR NOT NULL,
-- );
