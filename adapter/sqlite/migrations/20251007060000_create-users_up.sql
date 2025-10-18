create table users (
    id integer not null primary key autoincrement,
    email text unique not null,
    password text not null
);
