-- Your SQL goes here

CREATE TABLE routines (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    model INTEGER NOT NULL
);

CREATE TABLE models (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL
);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE tasklists (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    belongs_to INTEGER NOT NULL
);

CREATE TABLE tasklist_partof (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    tasklist INTEGER NOT NULL,
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