<script lang="ts">
  import { createEventDispatcher, onDestroy } from 'svelte';
  import { formatTime } from '../lib/time';

  export let src: string | null = null;
  export let currentTime = 0;
  export let duration = 0;

  const dispatch = createEventDispatcher<{
    loaded: { duration: number };
    time: { currentTime: number };
    playstate: { isPlaying: boolean };
  }>();

  let video: HTMLVideoElement;
  let rangeStopTimer: number | null = null;

  function handleLoadedMetadata() {
    duration = video.duration || 0;
    dispatch('loaded', { duration });
  }

  function handleTimeUpdate() {
    currentTime = video.currentTime;
    dispatch('time', { currentTime });
  }

  function clearRangeTimer() {
    if (rangeStopTimer !== null) {
      window.clearInterval(rangeStopTimer);
      rangeStopTimer = null;
    }
  }

  export function play() {
    void video?.play();
  }

  export function pause() {
    video?.pause();
  }

  export function togglePlay() {
    if (!video) return;
    if (video.paused) {
      void video.play();
    } else {
      video.pause();
    }
  }

  export function seek(time: number) {
    if (!video) return;
    video.currentTime = Math.min(Math.max(time, 0), video.duration || 0);
    handleTimeUpdate();
  }

  export function playSelectedRange(startTime: number, endTime: number) {
    if (!video) return;
    clearRangeTimer();
    seek(startTime);
    void video.play();
    rangeStopTimer = window.setInterval(() => {
      if (!video || video.currentTime >= endTime) {
        video.pause();
        clearRangeTimer();
      }
    }, 80);
  }

  onDestroy(clearRangeTimer);
</script>

<section class="preview-shell">
  {#if src}
    <!-- svelte-ignore a11y_media_has_caption -->
    <video
      bind:this={video}
      src={src}
      controls={false}
      playsinline
      on:loadedmetadata={handleLoadedMetadata}
      on:timeupdate={handleTimeUpdate}
      on:play={() => dispatch('playstate', { isPlaying: true })}
      on:pause={() => dispatch('playstate', { isPlaying: false })}
    ></video>
    <div class="time-readout">{formatTime(currentTime)} / {formatTime(duration)}</div>
  {:else}
    <div class="empty-preview">Video Preview</div>
  {/if}
</section>
