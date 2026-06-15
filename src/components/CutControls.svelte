<script lang="ts">
  import { formatTime } from '../lib/time';
  import type { CutMode } from '../lib/types';

  export let startTime: number | null = null;
  export let endTime: number | null = null;
  export let selectedDuration: number | null = null;
  export let outputPath: string | null = null;
  export let canEdit = false;
  export let canExport = false;
  export let isPlaying = false;
  export let isExporting = false;
  export let validationMessage: string | null = null;
  export let onTogglePlay: () => void;
  export let onPlaySelection: () => void;
  export let onMarkStart: () => void;
  export let onMarkEnd: () => void;
  export let onExport: (mode: CutMode, choosePath: boolean) => void;
</script>

<section class="controls">
  <div class="button-row">
    <button disabled={!canEdit || isExporting} on:click={onTogglePlay}>{isPlaying ? '일시정지' : '재생'}</button>
    <button disabled={!canExport || isExporting} on:click={onPlaySelection}>선택 구간 재생</button>
  </div>

  <div class="button-row">
    <button disabled={!canEdit || isExporting} on:click={onMarkStart}>시작점 지정</button>
    <button disabled={!canEdit || isExporting} on:click={onMarkEnd}>끝점 지정</button>
  </div>

  <dl class="cut-readout">
    <div>
      <dt>시작</dt>
      <dd>{formatTime(startTime)}</dd>
    </div>
    <div>
      <dt>끝</dt>
      <dd>{formatTime(endTime)}</dd>
    </div>
    <div>
      <dt>길이</dt>
      <dd>{formatTime(selectedDuration)}</dd>
    </div>
  </dl>

  {#if validationMessage}
    <p class="validation">{validationMessage}</p>
  {/if}

  <div class="output-line">
    <span>저장 파일</span>
    <strong>{outputPath ?? '파일을 열면 자동으로 정해집니다.'}</strong>
  </div>

  <div class="button-row export-row">
    <button class="primary" disabled={!canExport || isExporting} on:click={() => onExport('fast', false)}>
      {isExporting ? '자르는 중...' : '자르기'}
    </button>
    <button disabled={!canExport || isExporting} on:click={() => onExport('fast', true)}>다른 이름으로 저장</button>
  </div>
</section>
