create table users
(
    id          integer     not null primary key autoincrement,
    name        varchar     not null,
    last_name   varchar     not null,
    created_at  timestamp   not null default current_timestamp
)