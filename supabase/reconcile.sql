create extension if not exists pgcrypto;

create or replace function public.set_updated_at()
returns trigger
language plpgsql
as $$
begin
  new.updated_at = timezone('utc', now());
  return new;
end;
$$;

create table if not exists public.categories (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references auth.users(id) on delete cascade,
  name text not null,
  display_order integer not null default 0,
  created_at timestamptz not null default timezone('utc', now()),
  updated_at timestamptz not null default timezone('utc', now()),
  unique (user_id, name)
);

alter table public.categories
  add column if not exists user_id uuid references auth.users(id) on delete cascade,
  add column if not exists display_order integer default 0,
  add column if not exists created_at timestamptz default timezone('utc', now()),
  add column if not exists updated_at timestamptz default timezone('utc', now());

update public.categories
set
  display_order = coalesce(display_order, 0),
  created_at = coalesce(created_at, timezone('utc', now())),
  updated_at = coalesce(updated_at, timezone('utc', now()))
where
  display_order is null
  or created_at is null
  or updated_at is null;

create table if not exists public.todos (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references auth.users(id) on delete cascade,
  category_id uuid references public.categories(id) on delete cascade,
  text text not null,
  done boolean not null default false,
  display_order integer not null default 0,
  memo text,
  repeat_type text not null default 'none' check (repeat_type in ('none', 'daily', 'weekly', 'monthly')),
  repeat_detail text,
  next_due_at text,
  last_completed_at text,
  track_streak boolean not null default false,
  reminder_at text,
  linked_app text,
  created_at timestamptz not null default timezone('utc', now()),
  updated_at timestamptz not null default timezone('utc', now())
);

alter table public.todos
  add column if not exists user_id uuid references auth.users(id) on delete cascade,
  add column if not exists category_id uuid references public.categories(id) on delete cascade,
  add column if not exists display_order integer default 0,
  add column if not exists memo text,
  add column if not exists repeat_type text default 'none',
  add column if not exists repeat_detail text,
  add column if not exists next_due_at text,
  add column if not exists last_completed_at text,
  add column if not exists track_streak boolean default false,
  add column if not exists reminder_at text,
  add column if not exists linked_app text,
  add column if not exists created_at timestamptz default timezone('utc', now()),
  add column if not exists updated_at timestamptz default timezone('utc', now());

update public.todos
set
  display_order = coalesce(display_order, 0),
  repeat_type = coalesce(repeat_type, 'none'),
  track_streak = coalesce(track_streak, false),
  created_at = coalesce(created_at, timezone('utc', now())),
  updated_at = coalesce(updated_at, timezone('utc', now()))
where
  display_order is null
  or repeat_type is null
  or track_streak is null
  or created_at is null
  or updated_at is null;

do $$
begin
  if not exists (
    select 1
    from pg_constraint
    where conname = 'todos_repeat_type_check'
  ) then
    alter table public.todos
      add constraint todos_repeat_type_check
      check (repeat_type in ('none', 'daily', 'weekly', 'monthly'));
  end if;
end
$$;

create table if not exists public.completion_logs (
  id text primary key,
  user_id uuid not null references auth.users(id) on delete cascade,
  todo_id uuid not null references public.todos(id) on delete cascade,
  completed_on text not null,
  completed_count integer not null default 1
);

alter table public.completion_logs
  add column if not exists user_id uuid references auth.users(id) on delete cascade,
  add column if not exists todo_id uuid references public.todos(id) on delete cascade,
  add column if not exists completed_on text,
  add column if not exists completed_count integer default 1;

update public.completion_logs
set completed_count = coalesce(completed_count, 1)
where completed_count is null;

create table if not exists public.tags (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references auth.users(id) on delete cascade,
  name text not null,
  created_at timestamptz not null default timezone('utc', now()),
  updated_at timestamptz not null default timezone('utc', now()),
  unique (user_id, name)
);

alter table public.tags
  add column if not exists user_id uuid references auth.users(id) on delete cascade,
  add column if not exists created_at timestamptz default timezone('utc', now()),
  add column if not exists updated_at timestamptz default timezone('utc', now());

update public.tags
set
  created_at = coalesce(created_at, timezone('utc', now())),
  updated_at = coalesce(updated_at, timezone('utc', now()))
where created_at is null or updated_at is null;

create table if not exists public.todo_tags (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references auth.users(id) on delete cascade,
  todo_id uuid not null references public.todos(id) on delete cascade,
  tag_id uuid not null references public.tags(id) on delete cascade,
  created_at timestamptz not null default timezone('utc', now()),
  unique (todo_id, tag_id)
);

alter table public.todo_tags
  add column if not exists user_id uuid references auth.users(id) on delete cascade,
  add column if not exists created_at timestamptz default timezone('utc', now());

update public.todo_tags
set created_at = coalesce(created_at, timezone('utc', now()))
where created_at is null;

create index if not exists idx_categories_user_id on public.categories(user_id);
create index if not exists idx_todos_user_id on public.todos(user_id);
create index if not exists idx_todos_category_id on public.todos(category_id);
create index if not exists idx_completion_logs_user_id on public.completion_logs(user_id);
create index if not exists idx_completion_logs_todo_id on public.completion_logs(todo_id);
create index if not exists idx_tags_user_id on public.tags(user_id);
create index if not exists idx_todo_tags_user_id on public.todo_tags(user_id);
create index if not exists idx_todo_tags_todo_id on public.todo_tags(todo_id);
create index if not exists idx_todo_tags_tag_id on public.todo_tags(tag_id);

drop trigger if exists set_categories_updated_at on public.categories;
create trigger set_categories_updated_at
before update on public.categories
for each row
execute function public.set_updated_at();

drop trigger if exists set_todos_updated_at on public.todos;
create trigger set_todos_updated_at
before update on public.todos
for each row
execute function public.set_updated_at();

drop trigger if exists set_tags_updated_at on public.tags;
create trigger set_tags_updated_at
before update on public.tags
for each row
execute function public.set_updated_at();

alter table public.categories enable row level security;
alter table public.todos enable row level security;
alter table public.completion_logs enable row level security;
alter table public.tags enable row level security;
alter table public.todo_tags enable row level security;

drop policy if exists "Users can manage own categories" on public.categories;
create policy "Users can manage own categories"
on public.categories
for all
using (auth.uid() = user_id)
with check (auth.uid() = user_id);

drop policy if exists "Users can manage own todos" on public.todos;
create policy "Users can manage own todos"
on public.todos
for all
using (auth.uid() = user_id)
with check (auth.uid() = user_id);

drop policy if exists "Users can manage own completion_logs" on public.completion_logs;
create policy "Users can manage own completion_logs"
on public.completion_logs
for all
using (auth.uid() = user_id)
with check (auth.uid() = user_id);

drop policy if exists "Users can manage own tags" on public.tags;
create policy "Users can manage own tags"
on public.tags
for all
using (auth.uid() = user_id)
with check (auth.uid() = user_id);

drop policy if exists "Users can manage own todo_tags" on public.todo_tags;
create policy "Users can manage own todo_tags"
on public.todo_tags
for all
using (auth.uid() = user_id)
with check (auth.uid() = user_id);

do $$
declare
  table_name text;
begin
  if exists (select 1 from pg_publication where pubname = 'supabase_realtime') then
    foreach table_name in array array['categories', 'todos', 'completion_logs', 'tags', 'todo_tags']
    loop
      if not exists (
        select 1
        from pg_publication_rel pr
        join pg_class c on c.oid = pr.prrelid
        join pg_namespace n on n.oid = c.relnamespace
        join pg_publication p on p.oid = pr.prpubid
        where p.pubname = 'supabase_realtime'
          and n.nspname = 'public'
          and c.relname = table_name
      ) then
        execute format('alter publication supabase_realtime add table public.%I', table_name);
      end if;
    end loop;
  end if;
end
$$;
