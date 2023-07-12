-- Add migration script here
CREATE TABLE user_table (
    id serial primary key,
    email varchar(255) NOT NULL UNIQUE,
    password varchar(255) NOT NULL
)