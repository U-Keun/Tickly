<script lang="ts">
  import {
    ChevronRight,
    Clock3,
    Cloud,
    Languages,
    Palette,
    Tags,
    Type
  } from '@lucide/svelte';

  type MaybePromise = void | Promise<void>;
  type IconName = 'palette' | 'type' | 'languages' | 'clock' | 'tags' | 'cloud';
  type Tone = 'sky' | 'mint' | 'peach' | 'neutral';

  interface Props {
    icon: IconName;
    label: string;
    badge?: string | null;
    tone?: Tone;
    onSelect: () => MaybePromise;
  }

  let { icon, label, badge = null, tone = 'neutral', onSelect }: Props = $props();

  const icons = {
    palette: Palette,
    type: Type,
    languages: Languages,
    clock: Clock3,
    tags: Tags,
    cloud: Cloud
  };

  let Icon = $derived(icons[icon]);
  let iconClass = $derived.by(() => {
    if (tone === 'sky') return 'bg-accent-sky';
    if (tone === 'mint') return 'bg-accent-mint';
    if (tone === 'peach') return 'bg-accent-peach';
    return 'bg-canvas';
  });
</script>

<button
  type="button"
  class="group flex min-h-14 w-full min-w-0 items-center gap-3 border-t border-stroke bg-transparent px-3 py-2 text-left transition-colors first:border-t-0 hover:bg-canvas/70 active:bg-accent-sky/45"
  onclick={() => void onSelect()}
>
  <span
    class={`flex h-10 w-10 shrink-0 items-center justify-center rounded-[6px_14px_6px_14px] text-ink ${iconClass}`}
    aria-hidden="true"
  >
    <Icon size={20} strokeWidth={2.2} />
  </span>

  <span class="min-w-0 flex-1 truncate text-[16px] font-medium leading-6 text-ink">{label}</span>

  {#if badge}
    <span class="shrink-0 rounded-full border border-stroke bg-canvas px-2 py-0.5 text-[11px] font-semibold leading-4 text-ink-muted">
      {badge}
    </span>
  {/if}

  <ChevronRight
    class="shrink-0 text-ink-muted transition-transform group-hover:translate-x-0.5"
    size={19}
    strokeWidth={2.2}
    aria-hidden="true"
  />
</button>
