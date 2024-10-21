CREATE TABLE IF NOT EXISTS users
(
    id uuid default uuid_generate_v4() not null primary key,
    name text not null
);