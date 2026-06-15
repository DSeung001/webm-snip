<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import CutControls from './components/CutControls.svelte';
  import DropZone from './components/DropZone.svelte';
  import ErrorBanner from './components/ErrorBanner.svelte';
  import ExportStatus from './components/ExportStatus.svelte';
  import Timeline from './components/Timeline.svelte';
  import VideoPreview from './components/VideoPreview.svelte';
  import { clampTime } from './lib/time';
  import {
    chooseSavePath,
    cutWebM,
    generateDefaultOutputPath,
    openFile,
    openFolder,
    openWebMFile,
    probeMedia,
    toVideoSrc
  } from './lib/tauriApi';
  import type { AppError, AppStatus, CutMode, CutState, ExportResult, MediaInfo } from './lib/types';
  import { getCutDuration, validateCutRange } from './lib/validation';
  import { cutStore } from './stores/cutStore';
  import { exportStore } from './stores/exportStore';
  import { mediaStore } from './stores/mediaStore';

  let status: AppStatus = 'idle';
  let media: MediaInfo | null = null;
  let videoSrc: string | null = null;
  let currentTime = 0;
  let videoDuration = 0;
  let cut: CutState = { startTime: null, endTime: null, duration: null };
  let outputPath: string | null = null;
  let exportResult: ExportResult | null = null;
  let errorMessage: string | null = null;
  let isPlaying = false;
  let isDragActive = false;
  let videoRef: VideoPreview;

  $: selectedDuration = getCutDuration(cut.startTime, cut.endTime);
  $: validationMessage = media ? validateCutRange(cut, media.duration || videoDuration || null) : null;
  $: canEdit = Boolean(media) && status !== 'exporting';
  $: canExport = Boolean(media) && validationMessage === null && status !== 'exporting';
  $: {
    mediaStore.set(media);
    cutStore.set({ ...cut, duration: selectedDuration });
    exportStore.set(exportResult);
  }

  function userMessage(error: unknown): string {
    const appError = error as AppError;
    if (appError?.message) {
      return appError.message;
    }
    return '알 수 없는 문제가 발생했습니다.';
  }

  async function loadPath(path: string) {
    errorMessage = null;
    exportResult = null;
    status = 'loading_file';

    try {
      const nextMedia = await probeMedia(path);
      media = nextMedia;
      videoDuration = nextMedia.duration;
      videoSrc = toVideoSrc(nextMedia.path);
      currentTime = 0;
      cut = { startTime: null, endTime: null, duration: null };
      outputPath = await generateDefaultOutputPath(nextMedia.path);
      status = 'file_ready';
    } catch (error) {
      status = 'error';
      errorMessage = userMessage(error);
    }
  }

  async function openFileDialog() {
    try {
      const file = await openWebMFile();
      if (file) {
        await loadPath(file.path);
      }
    } catch (error) {
      status = 'error';
      errorMessage = userMessage(error);
    }
  }

  function markStart() {
    cut = { ...cut, startTime: currentTime };
    status = validationMessage ? 'invalid_selection' : 'ready_to_export';
  }

  function markEnd() {
    cut = { ...cut, endTime: currentTime };
    status = validationMessage ? 'invalid_selection' : 'ready_to_export';
  }

  function seek(time: number) {
    const duration = media?.duration || videoDuration || 0;
    videoRef?.seek(clampTime(time, duration));
  }

  function togglePlay() {
    videoRef?.togglePlay();
  }

  function playSelection() {
    if (!canExport || cut.startTime === null || cut.endTime === null) return;
    videoRef?.playSelectedRange(cut.startTime, cut.endTime);
  }

  async function runExport(mode: CutMode, choosePath = false, forcedPath: string | null = null) {
    if (!media || cut.startTime === null || cut.endTime === null || validationMessage) return;

    errorMessage = null;
    exportResult = null;
    status = 'exporting';

    try {
      const defaultPath = forcedPath ?? outputPath ?? (await generateDefaultOutputPath(media.path));
      const selectedPath = choosePath ? await chooseSavePath(defaultPath) : defaultPath;

      if (!selectedPath) {
        status = 'ready_to_export';
        return;
      }

      const result = await cutWebM({
        inputPath: media.path,
        outputPath: selectedPath,
        startTime: cut.startTime,
        endTime: cut.endTime,
        mode
      });

      exportResult = result;
      outputPath = await generateDefaultOutputPath(media.path);
      status = 'export_done';
    } catch (error) {
      status = 'error';
      errorMessage = userMessage(error);
    }
  }

  async function retryAccurate() {
    if (!media) return;
    const freshPath = await generateDefaultOutputPath(media.path);
    await runExport('accurate', false, freshPath);
  }

  function handleShortcut(event: KeyboardEvent) {
    if (!media || status === 'exporting') return;
    const element = event.target as HTMLElement | null;
    if (element?.tagName === 'INPUT' || element?.tagName === 'TEXTAREA') return;

    if (event.key === ' ') {
      event.preventDefault();
      togglePlay();
    } else if (event.key.toLowerCase() === 'i') {
      markStart();
    } else if (event.key.toLowerCase() === 'o') {
      markEnd();
    } else if (event.key === 'Enter' && canExport) {
      void runExport('fast', false);
    } else if (event.key === 'ArrowLeft') {
      event.preventDefault();
      seek(currentTime - (event.shiftKey ? 5 : 1));
    } else if (event.key === 'ArrowRight') {
      event.preventDefault();
      seek(currentTime + (event.shiftKey ? 5 : 1));
    }
  }

  onMount(() => {
    const unlistenPromise = listen<{ paths?: string[] }>('tauri://drag-drop', (event) => {
      const [path] = event.payload.paths ?? [];
      isDragActive = false;
      if (path) {
        void loadPath(path);
      }
    });

    const dragEnter = () => {
      isDragActive = true;
    };
    const dragLeave = () => {
      isDragActive = false;
    };

    window.addEventListener('keydown', handleShortcut);
    window.addEventListener('dragenter', dragEnter);
    window.addEventListener('dragleave', dragLeave);

    return () => {
      void unlistenPromise.then((unlisten) => unlisten());
      window.removeEventListener('keydown', handleShortcut);
      window.removeEventListener('dragenter', dragEnter);
      window.removeEventListener('dragleave', dragLeave);
    };
  });
</script>

<main class="app-shell">
  <header class="topbar">
    <div>
      <h1>WebM Snip</h1>
      <p>Open → Mark → Cut</p>
    </div>
    <button disabled={status === 'exporting'} on:click={openFileDialog}>WebM 파일 열기</button>
  </header>

  <ErrorBanner message={errorMessage} />

  {#if !media}
    <DropZone isActive={isDragActive} isLoading={status === 'loading_file'} onOpen={openFileDialog} />
  {:else}
    <div class="workspace">
      <VideoPreview
        bind:this={videoRef}
        bind:currentTime
        duration={videoDuration}
        src={videoSrc}
        on:loaded={(event) => (videoDuration = event.detail.duration)}
        on:time={(event) => (currentTime = event.detail.currentTime)}
        on:playstate={(event) => (isPlaying = event.detail.isPlaying)}
      />

      <Timeline
        currentTime={currentTime}
        disabled={status === 'exporting'}
        duration={media.duration || videoDuration}
        endTime={cut.endTime}
        onSeek={seek}
        startTime={cut.startTime}
      />

      <CutControls
        canEdit={canEdit}
        canExport={canExport}
        endTime={cut.endTime}
        isExporting={status === 'exporting'}
        isPlaying={isPlaying}
        onExport={runExport}
        onMarkEnd={markEnd}
        onMarkStart={markStart}
        onPlaySelection={playSelection}
        onTogglePlay={togglePlay}
        outputPath={outputPath}
        selectedDuration={selectedDuration}
        startTime={cut.startTime}
        validationMessage={validationMessage}
      />

      <ExportStatus
        isExporting={status === 'exporting'}
        onAccurateRetry={retryAccurate}
        onOpenFile={() => exportResult && openFile(exportResult.outputPath)}
        onOpenFolder={() => exportResult && openFolder(exportResult.outputPath)}
        result={exportResult}
      />
    </div>
  {/if}
</main>
