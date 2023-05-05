


CREATE TABLE user_table
(
    id INTEGER PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    first_name VARCHAR(255),
    second_name VARCHAR(255),
    last_name VARCHAR(255)
)