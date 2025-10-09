create table podcasts (
    id integer not null primary key autoincrement,
    feed_url text unique not null,
    title text not null,
    description text,
    image_url text,
    language text,
    website text,
    created_at integer not null default current_timestamp,
    updated_at integer not null default current_timestamp
);
