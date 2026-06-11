<script lang="ts">
  import { i18n } from '$lib/i18n';

  interface Props {
    label: string;
    value: string;
    onChange: (color: string) => void;
  }

  let { label, value, onChange }: Props = $props();

  let hexInput = $state('');

  $effect(() => {
    hexInput = value;
  });

  function handleColorInput(e: Event) {
    const target = e.target as HTMLInputElement;
    onChange(target.value);
  }

  function handleHexInput(e: Event) {
    const target = e.target as HTMLInputElement;
    let hex = target.value.trim();

    // Add # if missing
    if (hex && !hex.startsWith('#')) {
      hex = '#' + hex;
    }

    // Validate hex format
    if (/^#[0-9A-Fa-f]{6}$/.test(hex)) {
      onChange(hex);
    }

    hexInput = hex;
  }

  function handleHexBlur() {
    // Reset to current value if invalid
    if (!/^#[0-9A-Fa-f]{6}$/.test(hexInput)) {
      hexInput = value;
    }
  }
</script>

<div class="flex min-h-14 w-full min-w-0 items-center justify-between gap-3 border-t border-stroke px-3 py-2.5 first:border-t-0">
  <span class="min-w-0 flex-1 truncate text-[14px] font-semibold leading-5 text-ink">{label}</span>
  <div class="flex shrink-0 items-center gap-2">
    <div class="h-8 w-8 rounded-[5px_12px_5px_12px] border-2 border-stroke" style="background-color: {value}"></div>
    <input
      type="color"
      value={value}
      oninput={handleColorInput}
      class="h-8 w-9 cursor-pointer rounded-[5px_12px_5px_12px] border-0 bg-transparent p-0"
      aria-label={`${label} ${i18n.t('preview')}`}
    />
    <input
      type="text"
      value={hexInput}
      oninput={handleHexInput}
      onblur={handleHexBlur}
      class="h-9 w-[86px] rounded-[5px_12px_5px_12px] border-2 border-stroke bg-white px-2 font-mono text-[13px] font-semibold uppercase leading-5 text-ink outline-none focus:border-ink"
      placeholder="#000000"
      maxlength="7"
      aria-label={label}
    />
  </div>
</div>
