<script lang="ts">
  import { ArchiveRestore, FileText, Tags, Trash2 } from '@lucide/svelte';

  import SettingsGroup from './SettingsGroup.svelte';
  import SettingsShell from './SettingsShell.svelte';

  interface Props {
    state?: 'empty' | 'list';
  }

  let { state = 'list' }: Props = $props();

  const archivedItems = [
    {
      id: 1,
      text: 'Umbrella before leaving for a very long commute day',
      category: 'Home',
      archivedAt: '2026. 6. 16.',
      memo: 'Keep the compact one near the door.',
      tags: '#travel #rain'
    },
    {
      id: 2,
      text: 'Pay rent',
      category: 'Work',
      archivedAt: '2026. 6. 15.',
      memo: '',
      tags: '#money'
    }
  ];
</script>

<SettingsShell title="보관함 관리" onBack={() => {}}>
  <div class="flex flex-col gap-5">
    <SettingsGroup
      title="보관된 항목"
      description="완료 후 정리한 항목을 확인하고 다시 완료 목록으로 복원할 수 있어요."
    >
      {#if state === 'empty'}
        <div class="px-4 py-6 text-sm leading-6 text-ink-muted">
          <p class="font-semibold text-ink">보관된 항목 없음</p>
          <p class="mt-1">현재 보관함에 들어간 항목이 없어요.</p>
        </div>
      {:else}
        <div class="divide-y divide-stroke">
          {#each archivedItems as item (item.id)}
            <article class="min-w-0 px-3 py-3">
              <div class="flex min-w-0 items-start gap-3">
                <div
                  class="mt-1 flex h-10 w-10 shrink-0 items-center justify-center rounded-[6px_14px_6px_14px] bg-accent-sky text-ink"
                  aria-hidden="true"
                >
                  <ArchiveRestore size={20} strokeWidth={2.2} />
                </div>

                <div class="min-w-0 flex-1">
                  <div class="flex min-w-0 items-center gap-2">
                    <h2 class="min-w-0 flex-1 truncate text-[16px] font-semibold leading-6 text-ink">
                      {item.text}
                    </h2>
                    <span class="shrink-0 rounded-full bg-canvas px-2.5 py-1 text-[11px] font-semibold leading-4 text-ink-muted">
                      {item.category}
                    </span>
                  </div>

                  <p class="mt-1 text-xs font-semibold leading-5 text-ink-muted">
                    보관됨: {item.archivedAt}
                  </p>

                  {#if item.memo}
                    <p class="mt-2 max-h-10 overflow-hidden text-sm leading-5 text-ink-muted">
                      <FileText class="mr-1 inline-block align-[-2px]" size={14} strokeWidth={2.2} />
                      {item.memo}
                    </p>
                  {/if}

                  <p class="mt-1 truncate text-xs font-semibold leading-5 text-ink-muted">
                    <Tags class="mr-1 inline-block align-[-2px]" size={13} strokeWidth={2.2} />
                    {item.tags}
                  </p>
                </div>
              </div>

              <div class="mt-3 grid grid-cols-[1fr_auto] gap-2">
                <button
                  type="button"
                  class="min-h-11 rounded-[12px] bg-accent-sky-strong px-4 text-sm font-semibold text-ink transition-colors hover:bg-accent-sky"
                >
                  복원
                </button>

                <button
                  type="button"
                  class="flex min-h-11 w-12 items-center justify-center rounded-[12px] bg-canvas text-ink-muted transition-colors hover:bg-accent-peach"
                  aria-label="완전히 삭제"
                >
                  <Trash2 size={19} strokeWidth={2.2} aria-hidden="true" />
                </button>
              </div>
            </article>
          {/each}
        </div>
      {/if}
    </SettingsGroup>
  </div>
</SettingsShell>
