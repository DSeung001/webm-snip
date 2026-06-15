<script lang="ts">
  import { basename } from '../lib/filename';
  import { formatTime } from '../lib/time';
  import type { ExportResult } from '../lib/types';

  export let result: ExportResult | null = null;
  export let isExporting = false;
  export let onOpenFile: () => void;
  export let onOpenFolder: () => void;
  export let onAccurateRetry: () => void;
</script>

{#if isExporting || result}
  <section class="export-status" aria-live="polite">
    {#if isExporting}
      <strong>처리 중...</strong>
      <span>선택한 구간을 새 WebM 파일로 저장하고 있습니다.</span>
    {:else if result}
      <strong>자르기가 완료되었습니다.</strong>
      <span>{basename(result.outputPath)} · {formatTime(result.duration)}</span>
      <div class="button-row">
        <button on:click={onOpenFile}>파일 열기</button>
        <button on:click={onOpenFolder}>폴더 열기</button>
        <button on:click={onAccurateRetry}>정확하게 다시 자르기</button>
      </div>
    {/if}
  </section>
{/if}
