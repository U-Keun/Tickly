import type { RepeatType, Tag, TodoItem } from '../../types';
import * as streakApi from '../api/streakApi';
import * as tagApi from '../api/tagApi';
import * as todoApi from '../api/todoApi';

type ItemTagsMap = Record<number, Tag[]>;

interface ItemActionsContext {
  getItems: () => TodoItem[];
  setItems: (nextItems: TodoItem[]) => void;
  getSelectedCategoryId: () => number | null;
  getItemTagsMap: () => ItemTagsMap;
  setItemTagsMap: (nextMap: ItemTagsMap) => void;
  loadAllTags: () => Promise<void>;
  refreshAll: () => Promise<void>;
  finalizeMutation: () => Promise<void>;
  scheduleSync: () => void;
}

function sortItemsByDoneAndOrder(itemList: TodoItem[]): TodoItem[] {
  return [...itemList].sort((a, b) => {
    if (a.done !== b.done) {
      return a.done ? 1 : -1;
    }
    return a.display_order - b.display_order;
  });
}

function patchItem(itemList: TodoItem[], id: number, patch: Partial<TodoItem>): TodoItem[] {
  return itemList.map((item) => (item.id === id ? { ...item, ...patch } : item));
}

export function createItemActions(context: ItemActionsContext) {
  async function addItem(
    text: string,
    memo: string | null = null,
    repeatType: RepeatType = 'none',
    repeatDetail: string | null = null,
    trackStreak: boolean = false,
    tagNames: string[] = [],
    reminderAt: string | null = null,
    linkedApp: string | null = null
  ): Promise<void> {
    try {
      const newItem = await todoApi.addItem(
        text,
        context.getSelectedCategoryId(),
        repeatType,
        repeatDetail,
        trackStreak,
        reminderAt
      );

      if (memo) {
        await todoApi.updateItemMemo(newItem.id, memo);
        newItem.memo = memo;
      }

      if (linkedApp) {
        await todoApi.updateItemLinkedApp(newItem.id, linkedApp);
        newItem.linked_app = linkedApp;
      }

      context.setItems([...context.getItems(), newItem]);

      if (tagNames.length > 0) {
        const tags: Tag[] = [];
        for (const name of tagNames) {
          const tag = await tagApi.addTagToItem(newItem.id, name);
          tags.push(tag);
        }

        context.setItemTagsMap({
          ...context.getItemTagsMap(),
          [newItem.id]: tags
        });
        await context.loadAllTags();
      }

      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to add item:', error);
    }
  }

  async function toggleItem(id: number): Promise<void> {
    try {
      const updatedItem = await todoApi.toggleItem(id);
      if (!updatedItem) {
        return;
      }

      const nextItems = context
        .getItems()
        .map((item) => (item.id === id ? updatedItem : item));
      context.setItems(sortItemsByDoneAndOrder(nextItems));
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to toggle item:', error);
    }
  }

  async function deleteItem(id: number): Promise<void> {
    try {
      await todoApi.deleteItem(id);
      context.setItems(context.getItems().filter((item) => item.id !== id));
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to delete item:', error);
    }
  }

  async function editItem(id: number, text: string): Promise<void> {
    try {
      await todoApi.editItem(id, text);
      context.setItems(patchItem(context.getItems(), id, { text }));
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to edit item:', error);
    }
  }

  async function updateMemo(id: number, memo: string | null): Promise<void> {
    try {
      await todoApi.updateItemMemo(id, memo);
      context.setItems(patchItem(context.getItems(), id, { memo }));
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to update memo:', error);
    }
  }

  async function updateRepeat(
    id: number,
    repeatType: RepeatType,
    repeatDetail: string | null
  ): Promise<void> {
    try {
      await todoApi.updateItemRepeat(id, repeatType, repeatDetail);
      context.setItems(
        patchItem(context.getItems(), id, {
          repeat_type: repeatType,
          repeat_detail: repeatDetail
        })
      );
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to update repeat:', error);
    }
  }

  async function updateTrackStreak(id: number, trackStreak: boolean): Promise<void> {
    try {
      await streakApi.updateTrackStreak(id, trackStreak);
      context.setItems(patchItem(context.getItems(), id, { track_streak: trackStreak }));
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to update track_streak:', error);
    }
  }

  async function updateLinkedApp(id: number, linkedApp: string | null): Promise<void> {
    try {
      await todoApi.updateItemLinkedApp(id, linkedApp);
      context.setItems(patchItem(context.getItems(), id, { linked_app: linkedApp }));
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to update linked app:', error);
    }
  }

  async function updateReminder(id: number, reminderAt: string | null): Promise<void> {
    try {
      await todoApi.updateItemReminder(id, reminderAt);
      context.setItems(patchItem(context.getItems(), id, { reminder_at: reminderAt }));
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to update reminder:', error);
    }
  }

  async function resetAllItems(): Promise<void> {
    try {
      await todoApi.resetAllItems(context.getSelectedCategoryId());
      context.setItems(context.getItems().map((item) => ({ ...item, done: false })));
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to reset items:', error);
    }
  }

  function setItems(newItems: TodoItem[]): void {
    context.setItems(newItems);
  }

  return {
    addItem,
    toggleItem,
    deleteItem,
    editItem,
    updateMemo,
    updateRepeat,
    updateTrackStreak,
    updateLinkedApp,
    updateReminder,
    resetAllItems,
    setItems
  };
}
