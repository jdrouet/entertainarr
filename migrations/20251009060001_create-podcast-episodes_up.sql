create table podcast_episodes (
    id integer not null primary key autoincrement,
    podcast_id integer not null references podcasts(id) on delete cascade,
    guid text,
    published_at integer,
    title text not null,
    description text,
    link text,
    duration integer,
    file_url text not null,
    file_size integer,
    file_type string,
    created_at integer not null default current_timestamp,
    updated_at integer not null default current_timestamp,
    unique(podcast_id, guid)
);
