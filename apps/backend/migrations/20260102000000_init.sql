-- Migration: init
-- Created: 2026-01-02 00:00:00
-- Initial database schema

-- UP
create extension if not exists "uuid-ossp";

create enum user_role as ('admin', 'user', 'viewer');
create enum feature_flag_type as ('boolean', 'multivariate', 'json');
create enum value_type as ('string', 'number', 'boolean', 'json');
create enum match_operator as ('eq', 'neq', 'in', 'nin', 'gt', 'lt', 'gte', 'lte', 'contains', 'ncontains');
create enum audience_scope as ('global', 'inline');

create table users (
    id uuid default uuid_generate_v4() primary key,
    email varchar(255) not null unique,
    password_hash varchar(255) not null,
    role user_role not null default 'viewer',
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

create index idx_users_email on users(email);

create table magic_links (
    id uuid default uuid_generate_v4() primary key,
    user_id uuid references users(id) on delete cascade,
    token uuid default uuid_generate_v4() not null,
    expires_at timestamptz not null default (current_timestamp + interval '15 minutes'),
    created_at timestamptz default current_timestamp
);

create index idx_magic_links_token on magic_links(token);

create table projects (
    id uuid default uuid_generate_v4() primary key,
    name varchar(100) not null,
    description text,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

create table project_owners (
    project_id uuid references projects(id) on delete cascade,
    user_id uuid references users(id) on delete cascade,
    primary key (project_id, user_id)
);

create table environments (
    id uuid default uuid_generate_v4() primary key,
    project_id uuid references projects(id) on delete cascade,
    name varchar(100) not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

create index idx_environments_project_id on environments(project_id);

create table api_keys (
    id uuid default uuid_generate_v4() primary key,
    user_id uuid references users(id) on delete cascade,
    environment_id uuid references environments(id) on delete cascade,
    is_server_key boolean default false,
    key_hash varchar(255) not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

create index idx_api_keys_key_hash on api_keys(key_hash);

create table feature_flags (
    id uuid default uuid_generate_v4() primary key,
    key varchar(100) not null,
    is_enabled boolean default false,
    type feature_flag_type not null default 'boolean',
    value jsonb,
    project_id uuid references projects(id) on delete cascade,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,
    unique (key, project_id)
);

create index idx_feature_flags_key on feature_flags(key);

create table audiences (
    id uuid default uuid_generate_v4() primary key,
    name varchar(255) not null,
    attribute varchar(100) not null,
    operator match_operator not null,
    value varchar(255) not null,
    scope audience_scope not null,
    project_id uuid references projects(id) on delete cascade,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);
create index idx_audiences_name on audiences(name);

create table feature_flag_overrides (
    id uuid default uuid_generate_v4() primary key,
    feature_flag_id uuid references feature_flags(id) on delete cascade,
    audience_id uuid references audiences(id) on delete cascade,
    is_enabled boolean default false,
    type feature_flag_type not null default 'boolean',
    value jsonb,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,
    unique(feature_flag_id, audience_id)
);
create index idx_feature_flag_overrides_feature_flag_id on feature_flag_overrides(feature_flag_id);

-- DOWN
drop table if exists feature_flag_overrides;
drop table if exists audiences;
drop table if exists feature_flag_variants;
drop table if exists feature_flags;
drop table if exists api_keys;
drop table if exists environments;
drop table if exists project_owners;
drop table if exists projects;
drop table if exists magic_links;
drop table if exists users;
drop type if exists feature_flag_type;
drop type if exists user_role;
drop type if exists value_type;
drop type if exists match_operator;
drop type if exists audience_scope;
drop extension if exists "uuid-ossp";