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

create table if not exists public.completion_logs (
  id text primary key,
  user_id uuid not null references auth.users(id) on delete cascade,
  todo_id uuid not null references public.todos(id) on delete cascade,
  completed_on text not null,
  completed_count integer not null default 1
);

create table if not exists public.tags (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references auth.users(id) on delete cascade,
  name text not null,
  created_at timestamptz not null default timezone('utc', now()),
  updated_at timestamptz not null default timezone('utc', now()),
  unique (user_id, name)
);

create table if not exists public.todo_tags (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references auth.users(id) on delete cascade,
  todo_id uuid not null references public.todos(id) on delete cascade,
  tag_id uuid not null references public.tags(id) on delete cascade,
  created_at timestamptz not null default timezone('utc', now()),
  unique (todo_id, tag_id)
);

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
