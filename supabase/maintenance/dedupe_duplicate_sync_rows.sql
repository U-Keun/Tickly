-- Tickly remote duplicate cleanup
--
-- Run `supabase/maintenance/audit_duplicate_sync_rows.sql` first.
-- This script only removes:
-- 1) duplicate categories with the same (user_id, name)
-- 2) duplicate tags with the same (user_id, name)
-- 3) "safe exact duplicate" todos that share the same logical fields and created_at
-- 4) duplicate todo_tags with the same (user_id, todo_id, tag_id)
--
-- It also rewires foreign keys before deleting duplicate rows.

begin;

drop table if exists tmp_duplicate_categories;
drop table if exists tmp_duplicate_tags;
drop table if exists tmp_duplicate_todos;
drop table if exists tmp_duplicate_todo_tags;

create temporary table tmp_duplicate_categories as
with ranked as (
  select
    id,
    first_value(id) over (
      partition by user_id, name
      order by updated_at desc nulls last, created_at desc nulls last, id desc
    ) as keep_id,
    row_number() over (
      partition by user_id, name
      order by updated_at desc nulls last, created_at desc nulls last, id desc
    ) as rn
  from public.categories
)
select id as drop_id, keep_id
from ranked
where rn > 1
  and id is distinct from keep_id;

update public.todos t
set category_id = d.keep_id
from tmp_duplicate_categories d
where t.category_id = d.drop_id
  and t.category_id is distinct from d.keep_id;

delete from public.categories c
using tmp_duplicate_categories d
where c.id = d.drop_id;

create temporary table tmp_duplicate_tags as
with ranked as (
  select
    id,
    first_value(id) over (
      partition by user_id, name
      order by updated_at desc nulls last, created_at desc nulls last, id desc
    ) as keep_id,
    row_number() over (
      partition by user_id, name
      order by updated_at desc nulls last, created_at desc nulls last, id desc
    ) as rn
  from public.tags
)
select id as drop_id, keep_id
from ranked
where rn > 1
  and id is distinct from keep_id;

insert into public.todo_tags (id, user_id, todo_id, tag_id, created_at)
select
  gen_random_uuid(),
  min(tt.user_id::text)::uuid as user_id,
  tt.todo_id,
  d.keep_id,
  min(tt.created_at) as created_at
from public.todo_tags tt
join tmp_duplicate_tags d on d.drop_id = tt.tag_id
where not exists (
  select 1
  from public.todo_tags existing
  where existing.todo_id = tt.todo_id
    and existing.tag_id = d.keep_id
)
group by tt.todo_id, d.keep_id;

delete from public.todo_tags tt
using tmp_duplicate_tags d
where tt.tag_id = d.drop_id;

delete from public.tags t
using tmp_duplicate_tags d
where t.id = d.drop_id;

create temporary table tmp_duplicate_todos as
with ranked as (
  select
    id,
    first_value(id) over (
      partition by
        user_id,
        category_id,
        text,
        done,
        display_order,
        memo,
        repeat_type,
        repeat_detail,
        next_due_at,
        last_completed_at,
        track_streak,
        reminder_at,
        linked_app,
        created_at
      order by updated_at desc nulls last, id desc
    ) as keep_id,
    row_number() over (
      partition by
        user_id,
        category_id,
        text,
        done,
        display_order,
        memo,
        repeat_type,
        repeat_detail,
        next_due_at,
        last_completed_at,
        track_streak,
        reminder_at,
        linked_app,
        created_at
      order by updated_at desc nulls last, id desc
    ) as rn
  from public.todos
)
select id as drop_id, keep_id
from ranked
where rn > 1
  and id is distinct from keep_id;

insert into public.completion_logs (id, user_id, todo_id, completed_on, completed_count)
select
  d.keep_id::text || '_' || cl.completed_on as id,
  min(cl.user_id::text)::uuid as user_id,
  d.keep_id as todo_id,
  cl.completed_on,
  max(cl.completed_count) as completed_count
from public.completion_logs cl
join tmp_duplicate_todos d on d.drop_id = cl.todo_id
group by d.keep_id, cl.completed_on
on conflict (id) do update
set completed_count = greatest(public.completion_logs.completed_count, excluded.completed_count);

insert into public.todo_tags (id, user_id, todo_id, tag_id, created_at)
select
  gen_random_uuid(),
  min(tt.user_id::text)::uuid as user_id,
  d.keep_id as todo_id,
  tt.tag_id,
  min(tt.created_at) as created_at
from public.todo_tags tt
join tmp_duplicate_todos d on d.drop_id = tt.todo_id
where not exists (
  select 1
  from public.todo_tags existing
  where existing.todo_id = d.keep_id
    and existing.tag_id = tt.tag_id
)
group by d.keep_id, tt.tag_id;

delete from public.todo_tags tt
using tmp_duplicate_todos d
where tt.todo_id = d.drop_id;

delete from public.completion_logs cl
using tmp_duplicate_todos d
where cl.todo_id = d.drop_id;

delete from public.todos t
using tmp_duplicate_todos d
where t.id = d.drop_id;

create temporary table tmp_duplicate_todo_tags as
with ranked as (
  select
    id,
    first_value(id) over (
      partition by todo_id, tag_id
      order by created_at desc nulls last, id desc
    ) as keep_id,
    row_number() over (
      partition by todo_id, tag_id
      order by created_at desc nulls last, id desc
    ) as rn
  from public.todo_tags
)
select id as drop_id, keep_id
from ranked
where rn > 1
  and id is distinct from keep_id;

delete from public.todo_tags tt
using tmp_duplicate_todo_tags d
where tt.id = d.drop_id;

delete from public.completion_logs cl
where not exists (
  select 1
  from public.todos t
  where t.id = cl.todo_id
);

create unique index if not exists categories_user_id_name_key
  on public.categories(user_id, name);

create unique index if not exists tags_user_id_name_key
  on public.tags(user_id, name);

create unique index if not exists todo_tags_todo_id_tag_id_key
  on public.todo_tags(todo_id, tag_id);

select
  (select count(*) from tmp_duplicate_categories) as removed_categories,
  (select count(*) from tmp_duplicate_tags) as removed_tags,
  (select count(*) from tmp_duplicate_todos) as removed_todos,
  (select count(*) from tmp_duplicate_todo_tags) as removed_todo_tags;

commit;

drop table if exists tmp_duplicate_categories;
drop table if exists tmp_duplicate_tags;
drop table if exists tmp_duplicate_todos;
drop table if exists tmp_duplicate_todo_tags;
