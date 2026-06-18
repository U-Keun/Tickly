import type { RepeatType, Tag, TodoItem } from '../../types';
import * as nativeSheetApi from '../api/nativeSheetApi';
import { i18n } from '../i18n';
import {
  asRepeatType,
  parseRepeatDetail,
  stringifyRepeatDetail
} from './repeat';

export interface ItemDetailValues {
  id: number;
  text: string;
  memo: string | null;
  tagNames: string[];
  repeatType: RepeatType;
  repeatDetail: string | null;
  reminderAt: string | null;
  trackStreak: boolean;
}

export type ItemDetailLaunchResult =
  | { status: 'saved'; values: ItemDetailValues }
  | { status: 'cancelled' }
  | { status: 'unavailable' };

function parseNativeRepeatDetailValue(value: string | string[] | undefined): number[] {
  if (Array.isArray(value)) {
    return value.map((entry) => Number(entry)).filter((entry) => Number.isInteger(entry));
  }

  if (typeof value === 'string') {
    return parseRepeatDetail(value);
  }

  return [];
}

export async function openNativeItemDetailSheet(
  item: TodoItem,
  availableTags: Tag[]
): Promise<ItemDetailLaunchResult> {
  const nativeResult = await nativeSheetApi.openNativeFormSheet({
    title: i18n.t('checklistEditItemDetails'),
    fields: [
      {
        id: 'text',
        kind: 'text',
        label: i18n.t('checklistItemTextLabel'),
        placeholder: i18n.t('checklistItemTextPlaceholder'),
        initialValue: item.text,
        required: true
      },
      {
        id: 'memo',
        kind: 'textarea',
        label: i18n.t('checklistItemMemoLabel'),
        placeholder: i18n.t('checklistItemMemoPlaceholder'),
        initialValue: item.memo ?? ''
      },
      {
        id: 'tags',
        kind: 'tags',
        label: i18n.t('checklistItemTagsLabel'),
        placeholder: i18n.t('checklistItemTagsPlaceholder'),
        initialValue: '',
        initialTags: item.tags.map((tag) => tag.name),
        suggestions: availableTags.map((tag) => tag.name)
      },
      {
        id: 'repeat',
        kind: 'repeat',
        label: i18n.t('checklistItemRepeatLabel'),
        placeholder: i18n.t('checklistItemRepeatPlaceholder'),
        initialValue: item.repeat_type,
        initialRepeatDetail: parseRepeatDetail(item.repeat_detail),
        repeatLabels: {
          none: i18n.t('repeatNone'),
          daily: i18n.t('repeatDaily'),
          weekly: i18n.t('repeatWeekly'),
          monthly: i18n.t('repeatMonthly'),
          weeklyDetail: i18n.t('repeatDaysLabel'),
          monthlyDetail: i18n.t('repeatDatesLabel'),
          weekdays: [
            i18n.t('sun'),
            i18n.t('mon'),
            i18n.t('tue'),
            i18n.t('wed'),
            i18n.t('thu'),
            i18n.t('fri'),
            i18n.t('sat')
          ]
        }
      },
      {
        id: 'reminderAt',
        kind: 'time',
        label: i18n.t('checklistItemReminderLabel'),
        placeholder: i18n.t('checklistItemReminderPlaceholder'),
        initialValue: item.reminder_at ?? '',
        clearLabel: i18n.t('reminderClear')
      },
      {
        id: 'trackStreak',
        kind: 'toggle',
        label: i18n.t('checklistTrackStreak'),
        placeholder: '',
        initialValue: item.repeat_type !== 'none' && item.track_streak ? 'true' : 'false',
        requiresRepeat: true,
        disabledMessage: i18n.t('checklistTrackStreakRequiresRepeat')
      }
    ],
    confirmLabel: i18n.t('checklistSaveItem'),
    cancelLabel: i18n.t('cancel')
  });

  if (nativeResult.status === 'unavailable') {
    return { status: 'unavailable' };
  }

  if (nativeResult.status !== 'saved') {
    return { status: 'cancelled' };
  }

  const textValue = nativeResult.values.text;
  const memoValue = nativeResult.values.memo;
  const tagValues = nativeResult.values.tags;
  const repeatTypeValue = nativeResult.values.repeat;
  const repeatDetailValue = nativeResult.values.repeatDetail;
  const reminderAtValue = nativeResult.values.reminderAt;
  const trackStreakValue = nativeResult.values.trackStreak;
  const repeatType = asRepeatType(repeatTypeValue);
  const requestedTrackStreak =
    typeof trackStreakValue === 'string' ? trackStreakValue === 'true' : item.track_streak;
  const repeatDetail = stringifyRepeatDetail(
    repeatType,
    parseNativeRepeatDetailValue(repeatDetailValue)
  );

  return {
    status: 'saved',
    values: {
      id: item.id,
      text: typeof textValue === 'string' ? textValue : '',
      memo: typeof memoValue === 'string' && memoValue.trim() ? memoValue : null,
      tagNames: Array.isArray(tagValues) ? tagValues : [],
      repeatType,
      repeatDetail,
      reminderAt:
        typeof reminderAtValue === 'string' && reminderAtValue ? reminderAtValue : null,
      trackStreak: repeatType !== 'none' && requestedTrackStreak
    }
  };
}
