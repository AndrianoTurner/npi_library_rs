-- Add migration script here
CREATE TABLE user_table (
    id serial primary key,
    email varchar(255) NOT NULL UNIQUE,
    password varchar(255) NOT NULL
);

CREATE TABLE permissions(
    id varchar(255) NOT NULL primary key
);

CREATE TABLE roles(
    id varchar(255) NOT NULL primary key
);

create TABLE role_permissions(
    role_id varchar(255) NOT NULL,
    permission_id varchar(255) NOT NULL,
    PRIMARY KEY (role_id,permission_id)
);

CREATE TABLE user_roles(
    role_id varchar(255) NOT NULL,
    user_id serial NOT NULL,
    PRIMARY KEY (role_id,user_id)
);

INSERT INTO roles (id) VALUES ('none'), ('student'), ('administrator'), ('teacher');
INSERT INTO permissions (id) VALUES ('none'),('edit'),('view'),('full');
INSERT INTO role_permissions (role_id,permission_id) VALUES ('none','none'), ('student','view'), ('administrator','full'), ('teacher','edit');
