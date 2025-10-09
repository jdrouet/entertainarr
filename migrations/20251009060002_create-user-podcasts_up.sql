create table user_podcasts (
    user_id integer not null references users(id) on delete cascade,
    podcast_id integer not null references podcasts(id) on delete cascade,
    created_at integer not null default current_timestamp,
    primary key(user_id, podcast_id)
);
