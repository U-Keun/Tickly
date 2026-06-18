import type { V2RepeatType, V2Tag, V2TodoItem } from '../../types';
import * as v2NativeSheetApi from '../api/v2NativeSheetApi';
import { i18n } from '../i18n';
import {
  asV2RepeatType,
  parseV2RepeatDetail,
  stringifyV2RepeatDetail
} from './v2Repeat';

export interface V2ItemDetailValues {
  id: number;
  text: string;
  memo: string | null;
  tagNames: string[];
  repeatType: V2RepeatType;
  repeatDetail: string | null;
  reminderAt: string | null;
  trackStreak: boolean;
}

export type V2ItemDetailLaunchResult =
  | { status: 'saved'; values: V2ItemDetailValues }
  | { status: 'cancelled' }
  | { status: 'unavailable' };

function parseNativeRepeatDetailValue(value: string | string[] | undefined): number[] {
  if (Array.isArray(value)) {
    return value.map((entry) => Number(entry)).filter((entry) => Number.isInteger(entry));
  }

  if (typeof value === 'string') {
    return parseV2RepeatDetail(value);
  }

  return [];
}

export async function openNativeV2ItemDetailSheet(
  item: V2TodoItem,
  availableTags: V2Tag[]
): Promise<V2ItemDetailLaunchResult> {
  const nativeResult = await v2NativeSheetApi.openNativeFormSheet({
    title: i18n.t('v2EditItemDetails'),
    fields: [
      {
        id: 'text',
        kind: 'text',
        label: i18n.t('v2ItemTextLabel'),
        placeholder: i18n.t('v2ItemTextPlaceholder'),
        initialValue: item.text,
        required: true
      },
      {
        id: 'memo',
        kind: 'textarea',
        label: i18n.t('v2ItemMemoLabel'),
        placeholder: i18n.t('v2ItemMemoPlaceholder'),
        initialValue: item.memo ?? ''
      },
      {
        id: 'tags',
        kind: 'tags',
        label: i18n.t('v2ItemTagsLabel'),
        placeholder: i18n.t('v2ItemTagsPlaceholder'),
        initialValue: '',
        initialTags: item.tags.map((tag) => tag.name),
        suggestions: availableTags.map((tag) => tag.name)
      },
      {
        id: 'repeat',
        kind: 'repeat',
        label: i18n.t('v2ItemRepeatLabel'),
        placeholder: i18n.t('v2ItemRepeatPlaceholder'),
        initialValue: item.repeat_type,
        initialRepeatDetail: parseV2RepeatDetail(item.repeat_detail),
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
        label: i18n.t('v2ItemReminderLabel'),
        placeholder: i18n.t('v2ItemReminderPlaceholder'),
        initialValue: item.reminder_at ?? '',
        clearLabel: i18n.t('reminderClear')
      },
      {
        id: 'trackStreak',
        kind: 'toggle',
        label: i18n.t('v2TrackStreak'),
        placeholder: '',
        initialValue: item.repeat_type !== 'none' && item.track_streak ? 'true' : 'false',
        requiresRepeat: true,
        disabledMessage: i18n.t('v2TrackStreakRequiresRepeat')
      }
    ],
    confirmLabel: i18n.t('v2SaveItem'),
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
  const repeatType = asV2RepeatType(repeatTypeValue);
  const requestedTrackStreak =
    typeof trackStreakValue === 'string' ? trackStreakValue === 'true' : item.track_streak;
  const repeatDetail = stringifyV2RepeatDetail(
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
