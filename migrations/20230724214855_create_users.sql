-- Add migration script here
begin;
create table users (
    id serial primary key,
    name text not null unique,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

create table todos (
    id serial primary key,
    title text not null,
    completed boolean not null default false,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
commit;
