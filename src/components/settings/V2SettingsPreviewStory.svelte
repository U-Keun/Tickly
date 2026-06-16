<script lang="ts">
  import V2SettingsGroup from './V2SettingsGroup.svelte';
  import V2SettingsActionFooter from './V2SettingsActionFooter.svelte';
  import V2SettingsChoiceCard from './V2SettingsChoiceCard.svelte';
  import V2SettingsChoiceRow from './V2SettingsChoiceRow.svelte';
  import V2SettingsRow from './V2SettingsRow.svelte';
  import V2SettingsShell from './V2SettingsShell.svelte';
</script>

<V2SettingsShell title="설정" onBack={() => {}}>
  <div class="flex flex-col gap-5">
    <V2SettingsGroup title="표시">
      <V2SettingsRow icon="palette" tone="sky" label="테마 변경" onSelect={() => {}} />
      <V2SettingsRow icon="type" tone="peach" label="폰트 변경" onSelect={() => {}} />
      <V2SettingsRow icon="languages" tone="mint" label="언어 변경" onSelect={() => {}} />
    </V2SettingsGroup>

    <V2SettingsGroup title="하루 기준">
      <V2SettingsRow icon="clock" tone="peach" label="초기화 시간" onSelect={() => {}} />
    </V2SettingsGroup>

    <V2SettingsGroup title="데이터 관리">
      <V2SettingsRow icon="archive" tone="sky" label="보관함 관리" onSelect={() => {}} />
    </V2SettingsGroup>

    <V2SettingsGroup title="이전 기능">
      <V2SettingsRow icon="tags" tone="mint" label="태그 관리" badge="Legacy" onSelect={() => {}} />
      <V2SettingsRow icon="cloud" tone="sky" label="클라우드 동기화" badge="Legacy" onSelect={() => {}} />
    </V2SettingsGroup>

    <V2SettingsGroup title="선택 Row">
      <V2SettingsChoiceRow label="한국어" description="Korean" selected onSelect={() => {}} />
      <V2SettingsChoiceRow label="English" description="English" onSelect={() => {}} />
    </V2SettingsGroup>

    <section class="space-y-2">
      <h2 class="px-1 text-[13px] font-semibold leading-5 text-ink-muted">선택 Card</h2>
      <div class="grid grid-cols-3 gap-2">
        <V2SettingsChoiceCard label="작게" onSelect={() => {}}>
          <span class="block text-[22px] font-semibold leading-7 text-ink">14px</span>
        </V2SettingsChoiceCard>
        <V2SettingsChoiceCard label="보통" selected onSelect={() => {}}>
          <span class="block text-[22px] font-semibold leading-7 text-ink">16px</span>
        </V2SettingsChoiceCard>
        <V2SettingsChoiceCard label="크게" onSelect={() => {}}>
          <span class="block text-[22px] font-semibold leading-7 text-ink">18px</span>
        </V2SettingsChoiceCard>
      </div>
    </section>
  </div>

  {#snippet footer()}
    <V2SettingsActionFooter onSave={() => {}} />
  {/snippet}
</V2SettingsShell>
