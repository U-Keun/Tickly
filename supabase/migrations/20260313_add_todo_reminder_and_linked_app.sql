alter table public.todos
  add column if not exists reminder_at text;

alter table public.todos
  add column if not exists linked_app text;
