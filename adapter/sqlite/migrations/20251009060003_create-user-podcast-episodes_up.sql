create table user_podcast_episodes (
    user_id integer not null references users(id) on delete cascade,
    podcast_episode_id integer not null references podcast_episodes(id) on delete cascade,
    progress integer not null,
    completed boolean not null,
    created_at integer not null default current_timestamp,
    updated_at integer not null default current_timestamp,
    primary key(user_id, podcast_episode_id)
);
