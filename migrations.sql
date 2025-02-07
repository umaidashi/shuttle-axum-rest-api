-- migrations.sql
CREATE TABLE IF NOT EXISTS users (
    id serial primary key,
    name varchar not null,
    age int not null
);
