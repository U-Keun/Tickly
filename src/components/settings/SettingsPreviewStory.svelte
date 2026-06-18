<script lang="ts">
  import SettingsGroup from './SettingsGroup.svelte';
  import SettingsActionFooter from './SettingsActionFooter.svelte';
  import SettingsChoiceCard from './SettingsChoiceCard.svelte';
  import SettingsChoiceRow from './SettingsChoiceRow.svelte';
  import SettingsRow from './SettingsRow.svelte';
  import SettingsShell from './SettingsShell.svelte';
</script>

<SettingsShell title="설정" onBack={() => {}}>
  <div class="flex flex-col gap-5">
    <SettingsGroup title="표시">
      <SettingsRow icon="palette" tone="sky" label="테마 변경" onSelect={() => {}} />
      <SettingsRow icon="type" tone="peach" label="폰트 변경" onSelect={() => {}} />
      <SettingsRow icon="languages" tone="mint" label="언어 변경" onSelect={() => {}} />
    </SettingsGroup>

    <SettingsGroup title="하루 기준">
      <SettingsRow icon="clock" tone="peach" label="초기화 시간" onSelect={() => {}} />
    </SettingsGroup>

    <SettingsGroup title="데이터 관리">
      <SettingsRow icon="tags" tone="mint" label="태그 관리" onSelect={() => {}} />
      <SettingsRow icon="archive" tone="sky" label="보관함 관리" onSelect={() => {}} />
    </SettingsGroup>

    <SettingsGroup title="선택 Row">
      <SettingsChoiceRow label="한국어" description="Korean" selected onSelect={() => {}} />
      <SettingsChoiceRow label="English" description="English" onSelect={() => {}} />
    </SettingsGroup>

    <section class="space-y-2">
      <h2 class="px-1 text-[13px] font-semibold leading-5 text-ink-muted">선택 Card</h2>
      <div class="grid grid-cols-3 gap-2">
        <SettingsChoiceCard label="작게" onSelect={() => {}}>
          <span class="block text-[22px] font-semibold leading-7 text-ink">14px</span>
        </SettingsChoiceCard>
        <SettingsChoiceCard label="보통" selected onSelect={() => {}}>
          <span class="block text-[22px] font-semibold leading-7 text-ink">16px</span>
        </SettingsChoiceCard>
        <SettingsChoiceCard label="크게" onSelect={() => {}}>
          <span class="block text-[22px] font-semibold leading-7 text-ink">18px</span>
        </SettingsChoiceCard>
      </div>
    </section>
  </div>

  {#snippet footer()}
    <SettingsActionFooter onSave={() => {}} />
  {/snippet}
</SettingsShell>
