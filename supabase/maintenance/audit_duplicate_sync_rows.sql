-- Tickly remote duplicate audit
--
-- Run this first in Supabase SQL Editor to inspect duplicate groups before cleanup.
-- Todo duplicates here are "safe exact duplicates":
-- same logical fields and created_at, ignoring only id and updated_at.

-- Summary
with duplicate_categories as (
  select user_id, name, count(*) as duplicate_count
  from public.categories
  group by user_id, name
  having count(*) > 1
),
duplicate_tags as (
  select user_id, name, count(*) as duplicate_count
  from public.tags
  group by user_id, name
  having count(*) > 1
),
duplicate_todos as (
  select
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
    created_at,
    count(*) as duplicate_count
  from public.todos
  group by
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
  having count(*) > 1
),
duplicate_todo_tags as (
  select todo_id, tag_id, count(*) as duplicate_count
  from public.todo_tags
  group by todo_id, tag_id
  having count(*) > 1
)
select 'duplicate_category_groups' as issue, count(*) as groups, coalesce(sum(duplicate_count - 1), 0) as extra_rows
from duplicate_categories
union all
select 'duplicate_tag_groups', count(*), coalesce(sum(duplicate_count - 1), 0)
from duplicate_tags
union all
select 'duplicate_todo_groups', count(*), coalesce(sum(duplicate_count - 1), 0)
from duplicate_todos
union all
select 'duplicate_todo_tag_groups', count(*), coalesce(sum(duplicate_count - 1), 0)
from duplicate_todo_tags
union all
select 'orphan_completion_logs', count(*), count(*)
from public.completion_logs cl
left join public.todos t on t.id = cl.todo_id
where t.id is null
order by issue;

-- Duplicate categories by same user/name
select
  user_id,
  name,
  count(*) as duplicate_count,
  array_agg(id order by updated_at desc nulls last, created_at desc nulls last, id desc) as category_ids
from public.categories
group by user_id, name
having count(*) > 1
order by duplicate_count desc, name asc;

-- Duplicate tags by same user/name
select
  user_id,
  name,
  count(*) as duplicate_count,
  array_agg(id order by updated_at desc nulls last, created_at desc nulls last, id desc) as tag_ids
from public.tags
group by user_id, name
having count(*) > 1
order by duplicate_count desc, name asc;

-- Safe exact duplicate todos
select
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
  created_at,
  count(*) as duplicate_count,
  array_agg(id order by updated_at desc nulls last, id desc) as todo_ids
from public.todos
group by
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
having count(*) > 1
order by duplicate_count desc, created_at desc nulls last, text asc;

-- Duplicate todo_tags
select
  min(user_id::text)::uuid as user_id,
  todo_id,
  tag_id,
  count(*) as duplicate_count,
  array_agg(id order by created_at desc nulls last, id desc) as todo_tag_ids
from public.todo_tags
group by todo_id, tag_id
having count(*) > 1
order by duplicate_count desc, todo_id asc, tag_id asc;

-- Completion logs that reference a missing todo
select cl.*
from public.completion_logs cl
left join public.todos t on t.id = cl.todo_id
where t.id is null
order by cl.completed_on desc, cl.id asc;
