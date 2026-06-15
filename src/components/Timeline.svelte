<script lang="ts">
  import { formatTime } from '../lib/time';

  export let currentTime = 0;
  export let duration = 0;
  export let startTime: number | null = null;
  export let endTime: number | null = null;
  export let disabled = false;
  export let onSeek: (time: number) => void;

  $: startPct = duration > 0 && startTime !== null ? `${(startTime / duration) * 100}%` : null;
  $: endPct = duration > 0 && endTime !== null ? `${(endTime / duration) * 100}%` : null;
</script>

<section class="timeline">
  <div class="timeline-labels">
    <span>{formatTime(currentTime)}</span>
    <span>{formatTime(duration)}</span>
  </div>
  <div class="range-wrap">
    {#if startPct}
      <span class="marker start" style:left={startPct}></span>
    {/if}
    {#if endPct}
      <span class="marker end" style:left={endPct}></span>
    {/if}
    <input
      aria-label="재생 위치"
      disabled={disabled || duration <= 0}
      max={duration}
      min="0"
      step="0.001"
      type="range"
      value={currentTime}
      on:input={(event) => onSeek(Number((event.currentTarget as HTMLInputElement).value))}
    />
  </div>
</section>
