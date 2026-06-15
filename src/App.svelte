<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount, tick } from 'svelte';
  import ErrorBanner from './components/ErrorBanner.svelte';
  import VideoPreview from './components/VideoPreview.svelte';
  import { clampTime, formatTime } from './lib/time';
  import {
    chooseSavePath,
    exportTimeline,
    generateDefaultOutputPath,
    openFile,
    openFolder,
    openWebMFiles,
    probeMedia,
    toVideoSrc
  } from './lib/tauriApi';
  import type {
    AppError,
    AppStatus,
    ExportMode,
    ExportResult,
    MediaInfo,
    TimelineSegment
  } from './lib/types';

  type Clip = TimelineSegment & {
    id: number;
  };

  const MIN_CLIP_SECONDS = 0.08;

  let status: AppStatus = 'idle';
  let files: MediaInfo[] = [];
  let clips: Clip[] = [];
  let activeIndex = 0;
  let selectedClipId: number | null = null;
  let videoSrc: string | null = null;
  let currentTime = 0;
  let videoDuration = 0;
  let outputPath: string | null = null;
  let exportResult: ExportResult | null = null;
  let exportMode: ExportMode = 'fast';
  let errorMessage: string | null = null;
  let isPlaying = false;
  let isDragActive = false;
  let nextClipId = 1;
  let pendingSeekTime: number | null = null;
  let isScrubbingTimeline = false;
  let trackStrip: HTMLDivElement | undefined;
  let videoRef: VideoPreview | undefined;

  $: activeFile = files[activeIndex] ?? null;
  $: selectedClip = clips.find((clip) => clip.id === selectedClipId) ?? null;
  $: selectedClipIndex = selectedClip ? clips.findIndex((clip) => clip.id === selectedClip.id) : -1;
  $: selectedClipFile = selectedClip ? files[selectedClip.inputIndex] ?? null : null;
  $: selectedClipDuration = selectedClip ? clipDuration(selectedClip) : 0;
  $: totalDuration = clips.reduce((total, clip) => total + clipDuration(clip), 0);
  $: canEdit = selectedClip !== null && status !== 'exporting';
  $: canScrub = clips.length > 0 && status !== 'exporting';
  $: canExport = clips.length > 0 && status !== 'exporting';
  $: playheadTime = selectedClip
    ? timelineTimeForClip(selectedClip, currentTime)
    : 0;
  $: playheadPercent = totalDuration > 0 ? clampPercent((playheadTime / totalDuration) * 100) : 0;

  function userMessage(error: unknown): string {
    const appError = error as AppError;
    return appError?.message ?? 'Something went wrong.';
  }

  function createClip(inputIndex: number, startTime: number, endTime: number): Clip {
    return {
      id: nextClipId++,
      inputIndex,
      startTime,
      endTime
    };
  }

  function clipDuration(clip: TimelineSegment): number {
    return Math.max(0, clip.endTime - clip.startTime);
  }

  function clipOffset(clipId: number): number {
    let offset = 0;

    for (const clip of clips) {
      if (clip.id === clipId) return offset;
      offset += clipDuration(clip);
    }

    return offset;
  }

  function clampClipTime(clip: TimelineSegment, time: number): number {
    return Math.min(Math.max(time, clip.startTime), clip.endTime);
  }

  function timelineTimeForClip(clip: Clip, time: number): number {
    return clipOffset(clip.id) + clampClipTime(clip, time) - clip.startTime;
  }

  function findTimelineTarget(time: number): { clip: Clip; localTime: number } | null {
    const safeTime = clampTime(time, totalDuration);
    let cursor = 0;

    for (const [index, clip] of clips.entries()) {
      const duration = clipDuration(clip);
      const isLastClip = index === clips.length - 1;
      const clipEnd = cursor + duration;

      if (safeTime < clipEnd || isLastClip) {
        return {
          clip,
          localTime: clampClipTime(clip, clip.startTime + safeTime - cursor)
        };
      }

      cursor = clipEnd;
    }

    return null;
  }

  function clipName(clip: Clip): string {
    const file = files[clip.inputIndex];
    return file ? file.filename : `Clip ${clip.id}`;
  }

  function clipTitle(clip: Clip): string {
    return `${clipName(clip)} | ${formatTime(clip.startTime)} - ${formatTime(clip.endTime)}`;
  }

  function clampPercent(value: number): number {
    return Math.min(100, Math.max(0, value));
  }

  function isTauriRuntime(): boolean {
    return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
  }

  function resetProject() {
    status = 'idle';
    files = [];
    clips = [];
    activeIndex = 0;
    selectedClipId = null;
    videoSrc = null;
    currentTime = 0;
    videoDuration = 0;
    outputPath = null;
    exportResult = null;
    errorMessage = null;
    isPlaying = false;
    nextClipId = 1;
  }

  async function loadPaths(paths: string[], append = false) {
    const webmPaths = paths.filter((path) => path.toLowerCase().endsWith('.webm'));

    if (webmPaths.length === 0) {
      errorMessage = 'Only WebM files can be opened.';
      return;
    }

    errorMessage = null;
    exportResult = null;
    status = 'loading';

    try {
      const loadedFiles: MediaInfo[] = [];
      for (const path of webmPaths) {
        loadedFiles.push(await probeMedia(path));
      }

      if (!append) {
        nextClipId = 1;
      }

      const fileOffset = append ? files.length : 0;
      const newClips = loadedFiles.map((file, index) =>
        createClip(fileOffset + index, 0, file.duration)
      );

      files = append ? [...files, ...loadedFiles] : loadedFiles;
      clips = append ? [...clips, ...newClips] : newClips;
      status = 'ready';

      if (!append || !outputPath) {
        outputPath = await generateDefaultOutputPath(files[0].path);
      }

      const firstNewClip = newClips[0] ?? null;
      if (firstNewClip) {
        await showClip(firstNewClip, firstNewClip.startTime);
      }
    } catch (error) {
      status = 'error';
      errorMessage = userMessage(error);
    }
  }

  async function openFilesDialog(append = files.length > 0) {
    try {
      const selected = await openWebMFiles();
      if (selected.length === 0) return;
      await loadPaths(
        selected.map((file) => file.path),
        append
      );
    } catch (error) {
      status = 'error';
      errorMessage = userMessage(error);
    }
  }

  async function selectFile(index: number) {
    const firstClip = clips.find((clip) => clip.inputIndex === index);
    if (firstClip) {
      await showClip(firstClip, firstClip.startTime);
      return;
    }

    const file = files[index];
    if (!file) return;

    selectedClipId = null;
    activeIndex = index;
    videoDuration = file.duration;
    videoSrc = toVideoSrc(file.path);
    currentTime = 0;
    pendingSeekTime = 0;
  }

  async function showClip(clip: Clip, seekTime: number) {
    const file = files[clip.inputIndex];
    if (!file) return;

    const safeTime = Math.min(Math.max(seekTime, clip.startTime), clip.endTime);
    const changedFile = activeIndex !== clip.inputIndex;

    selectedClipId = clip.id;
    activeIndex = clip.inputIndex;
    videoDuration = file.duration;
    videoSrc = toVideoSrc(file.path);
    currentTime = safeTime;
    pendingSeekTime = safeTime;

    await tick();
    videoRef?.seek(safeTime);

    if (!changedFile) {
      pendingSeekTime = null;
    }
  }

  function handlePreviewLoaded(event: CustomEvent<{ duration: number }>) {
    videoDuration = event.detail.duration;

    if (pendingSeekTime !== null) {
      const seekTime = pendingSeekTime;
      pendingSeekTime = null;
      void tick().then(() => videoRef?.seek(seekTime));
    }
  }

  function seekPreview(time: number) {
    const duration = activeFile?.duration ?? videoDuration;
    videoRef?.seek(clampTime(time, duration));
  }

  function seekInsideSelectedClip(time: number) {
    if (!selectedClip) {
      seekPreview(time);
      return;
    }

    videoRef?.seek(clampClipTime(selectedClip, time));
  }

  function handlePlayheadInput(event: Event) {
    const value = Number((event.currentTarget as HTMLInputElement).value);
    if (Number.isFinite(value)) {
      seekInsideSelectedClip(value);
    }
  }

  function timelineTimeFromClientX(clientX: number): number | null {
    const rect = trackStrip?.getBoundingClientRect();
    if (!rect || rect.width <= 0) return null;

    const ratio = clampPercent(((clientX - rect.left) / rect.width) * 100) / 100;
    return totalDuration * ratio;
  }

  async function scrubTimelineAtClientX(clientX: number) {
    const time = timelineTimeFromClientX(clientX);
    if (time === null) return;
    await seekTimeline(time);
  }

  async function seekTimeline(time: number) {
    const target = findTimelineTarget(time);
    if (!target) return;
    await showClip(target.clip, target.localTime);
  }

  function stepTimeline(seconds: number) {
    void seekTimeline(playheadTime + seconds);
  }

  function beginTimelineScrub(event: PointerEvent) {
    if (!canScrub || !trackStrip) return;

    event.preventDefault();
    isScrubbingTimeline = true;
    videoRef?.pause();
    trackStrip.setPointerCapture?.(event.pointerId);
    void scrubTimelineAtClientX(event.clientX);
  }

  function updateTimelineScrub(event: PointerEvent) {
    if (!isScrubbingTimeline) return;

    event.preventDefault();
    void scrubTimelineAtClientX(event.clientX);
  }

  function endTimelineScrub(event: PointerEvent) {
    if (!isScrubbingTimeline) return;

    event.preventDefault();
    isScrubbingTimeline = false;
    if (trackStrip?.hasPointerCapture?.(event.pointerId)) {
      trackStrip.releasePointerCapture(event.pointerId);
    }
    void scrubTimelineAtClientX(event.clientX);
  }

  function handleTrackKeydown(event: KeyboardEvent) {
    if (!canScrub) return;

    if (event.key === 'ArrowLeft') {
      event.preventDefault();
      event.stopPropagation();
      stepTimeline(event.shiftKey ? -5 : -1);
    } else if (event.key === 'ArrowRight') {
      event.preventDefault();
      event.stopPropagation();
      stepTimeline(event.shiftKey ? 5 : 1);
    } else if (event.key === 'Home') {
      event.preventDefault();
      event.stopPropagation();
      void seekTimeline(0);
    } else if (event.key === 'End') {
      event.preventDefault();
      event.stopPropagation();
      void seekTimeline(totalDuration);
    }
  }

  function toggleClipPlayback() {
    if (isPlaying) {
      videoRef?.pause();
      return;
    }

    if (selectedClip) {
      const startTime =
        currentTime >= selectedClip.startTime && currentTime < selectedClip.endTime - MIN_CLIP_SECONDS
          ? currentTime
          : selectedClip.startTime;
      videoRef?.playSelectedRange(startTime, selectedClip.endTime);
    } else {
      videoRef?.togglePlay();
    }
  }

  async function splitSelectedClip() {
    if (!selectedClip) {
      errorMessage = 'Select a clip first.';
      return;
    }

    const splitTime = currentTime;
    const isInside =
      splitTime > selectedClip.startTime + MIN_CLIP_SECONDS &&
      splitTime < selectedClip.endTime - MIN_CLIP_SECONDS;

    if (!isInside) {
      errorMessage = 'Move the playhead inside the selected clip.';
      return;
    }

    const leftClip: Clip = { ...selectedClip, endTime: splitTime };
    const rightClip = createClip(selectedClip.inputIndex, splitTime, selectedClip.endTime);

    clips = clips.flatMap((clip) => (clip.id === selectedClip.id ? [leftClip, rightClip] : [clip]));
    errorMessage = null;
    await showClip(rightClip, rightClip.startTime);
  }

  async function deleteSelectedClip() {
    if (!selectedClip) {
      errorMessage = 'Select a clip first.';
      return;
    }

    const nextClips = clips.filter((clip) => clip.id !== selectedClip.id);
    const nextClip = nextClips[Math.min(selectedClipIndex, nextClips.length - 1)] ?? null;
    clips = nextClips;
    errorMessage = null;

    if (nextClip) {
      await showClip(nextClip, nextClip.startTime);
    } else {
      selectedClipId = null;
      currentTime = 0;
    }
  }

  function moveSelectedClip(direction: -1 | 1) {
    if (!selectedClip || selectedClipIndex < 0) return;

    const targetIndex = selectedClipIndex + direction;
    if (targetIndex < 0 || targetIndex >= clips.length) return;

    const nextClips = [...clips];
    [nextClips[selectedClipIndex], nextClips[targetIndex]] = [
      nextClips[targetIndex],
      nextClips[selectedClipIndex]
    ];
    clips = nextClips;
  }

  async function chooseOutputPath() {
    const firstFile = files[0];
    if (!firstFile) return;

    try {
      const defaultPath = outputPath ?? (await generateDefaultOutputPath(firstFile.path));
      const selected = await chooseSavePath(defaultPath);
      if (selected) {
        outputPath = selected;
      }
    } catch (error) {
      errorMessage = userMessage(error);
    }
  }

  async function runExport() {
    const firstFile = files[0];

    if (!firstFile || clips.length === 0) {
      errorMessage = 'Add at least one clip to the video line.';
      return;
    }

    errorMessage = null;
    exportResult = null;
    status = 'exporting';

    try {
      const targetPath = outputPath ?? (await generateDefaultOutputPath(firstFile.path));
      outputPath = targetPath;
      exportResult = await exportTimeline({
        inputPaths: files.map((file) => file.path),
        outputPath: targetPath,
        segments: clips.map(({ inputIndex, startTime, endTime }) => ({
          inputIndex,
          startTime,
          endTime
        })),
        mode: exportMode
      });
      status = 'done';
      outputPath = await generateDefaultOutputPath(firstFile.path);
    } catch (error) {
      status = 'error';
      errorMessage = userMessage(error);
    }
  }

  async function openExportedFile() {
    if (!exportResult) return;
    try {
      await openFile(exportResult.outputPath);
    } catch (error) {
      errorMessage = userMessage(error);
    }
  }

  async function openExportFolder() {
    if (!exportResult) return;
    try {
      await openFolder(exportResult.outputPath);
    } catch (error) {
      errorMessage = userMessage(error);
    }
  }

  function handleShortcut(event: KeyboardEvent) {
    if (status === 'exporting') return;

    const element = event.target as HTMLElement | null;
    if (element?.tagName === 'INPUT' || element?.tagName === 'TEXTAREA' || element?.tagName === 'SELECT') {
      return;
    }

    if (event.key === ' ') {
      event.preventDefault();
      toggleClipPlayback();
    } else if (event.key.toLowerCase() === 's') {
      event.preventDefault();
      void splitSelectedClip();
    } else if (event.key === 'Backspace' || event.key === 'Delete') {
      event.preventDefault();
      void deleteSelectedClip();
    } else if (event.key === 'Enter' && canExport) {
      event.preventDefault();
      void runExport();
    } else if (event.key === 'ArrowLeft') {
      event.preventDefault();
      stepTimeline(event.shiftKey ? -5 : -1);
    } else if (event.key === 'ArrowRight') {
      event.preventDefault();
      stepTimeline(event.shiftKey ? 5 : 1);
    }
  }

  onMount(() => {
    const unlistenPromise = isTauriRuntime()
      ? listen<{ paths?: string[] }>('tauri://drag-drop', (event) => {
          isDragActive = false;
          void loadPaths(event.payload.paths ?? [], files.length > 0);
        })
      : null;

    const showDropState = () => (isDragActive = true);
    const hideDropState = () => (isDragActive = false);

    window.addEventListener('keydown', handleShortcut);
    window.addEventListener('dragenter', showDropState);
    window.addEventListener('dragleave', hideDropState);

    return () => {
      if (unlistenPromise) {
        void unlistenPromise.then((unlisten) => unlisten());
      }
      window.removeEventListener('keydown', handleShortcut);
      window.removeEventListener('dragenter', showDropState);
      window.removeEventListener('dragleave', hideDropState);
    };
  });
</script>

<main class="app-shell">
  <header class="topbar">
    <div>
      <h1>WebM Snip</h1>
      <p>Split. Delete. Connect.</p>
    </div>
    <div class="topbar-actions">
      {#if files.length > 0}
        <button disabled={status === 'exporting'} on:click={() => openFilesDialog(true)}>Add WebM</button>
      {/if}
      <button disabled={status === 'exporting'} on:click={() => openFilesDialog(false)}>Open WebM</button>
    </div>
  </header>

  <ErrorBanner message={errorMessage} />

  {#if files.length === 0}
    <section class:is-active={isDragActive} class="drop-zone">
      <div>
        <h2>Drop WebM files here.</h2>
        <p>They will appear as clips on one video line.</p>
      </div>
      <button class="primary" disabled={status === 'loading'} on:click={() => openFilesDialog(false)}>
        Open WebM
      </button>
    </section>
  {:else}
    <div class="workspace">
      <section class="main-column">
        <VideoPreview
          bind:this={videoRef}
          bind:currentTime
          duration={videoDuration}
          src={videoSrc}
          on:loaded={handlePreviewLoaded}
          on:time={(event) => (currentTime = event.detail.currentTime)}
          on:playstate={(event) => (isPlaying = event.detail.isPlaying)}
        />

        <section class="panel video-line">
          <div class="panel-title">
            <div>
              <h2>Video Line</h2>
              <span>{clips.length} clips / {formatTime(totalDuration)}</span>
            </div>
            <button disabled={status === 'exporting'} on:click={() => openFilesDialog(true)}>Add WebM</button>
          </div>

          <div class="track-shell">
            <div class="track-ruler">
              <span>00:00:00.000</span>
              <strong>{formatTime(playheadTime)}</strong>
              <span>{formatTime(totalDuration)}</span>
            </div>
            <div class="timeline-controls">
              <button disabled={!canEdit} on:click={() => stepTimeline(-5)}>-5s</button>
              <button disabled={!canEdit} on:click={() => stepTimeline(-1)}>-1s</button>
              <button disabled={!canEdit} on:click={() => stepTimeline(1)}>+1s</button>
              <button disabled={!canEdit} on:click={() => stepTimeline(5)}>+5s</button>
            </div>
            <div
              aria-label="Video line position"
              aria-valuemax={totalDuration}
              aria-valuemin="0"
              aria-valuenow={playheadTime}
              aria-valuetext={`${formatTime(playheadTime)} / ${formatTime(totalDuration)}`}
              bind:this={trackStrip}
              class:scrubbing={isScrubbingTimeline}
              class="track-strip"
              on:keydown={handleTrackKeydown}
              on:pointercancel={endTimelineScrub}
              on:pointerdown={beginTimelineScrub}
              on:pointermove={updateTimelineScrub}
              on:pointerup={endTimelineScrub}
              role="slider"
              tabindex="0"
            >
              {#each clips as clip (clip.id)}
                <div
                  class="track-clip"
                  class:selected={clip.id === selectedClipId}
                  style={`flex: ${Math.max(clipDuration(clip), 0.2)} 1 0px`}
                  title={clipTitle(clip)}
                >
                  <span>{clipName(clip)}</span>
                  <small>{formatTime(clipDuration(clip))}</small>
                </div>
              {/each}
              {#if clips.length > 0}
                <div class="track-playhead" style={`left: ${playheadPercent}%`}></div>
              {/if}
            </div>
          </div>
        </section>
      </section>

      <aside class="side-column">
        <section class="panel">
          <div class="panel-title">
            <div>
              <h2>Clip</h2>
              <span>{isPlaying ? 'Playing' : 'Paused'}</span>
            </div>
          </div>

          {#if selectedClip}
            <div class="clip-heading">
              <strong>{clipName(selectedClip)}</strong>
              <span>{formatTime(selectedClipDuration)}</span>
            </div>

            <div class="transport-grid">
              <button disabled={!canEdit} on:click={toggleClipPlayback}>{isPlaying ? 'Pause' : 'Play'}</button>
              <button class="primary" disabled={!canEdit} on:click={splitSelectedClip}>Split</button>
              <button disabled={!canEdit} on:click={deleteSelectedClip}>Delete</button>
            </div>

            <label class="field">
              <span>Playhead</span>
              <input
                disabled={!canEdit}
                max={selectedClip.endTime}
                min={selectedClip.startTime}
                on:change={handlePlayheadInput}
                step="0.001"
                type="number"
                value={currentTime.toFixed(3)}
              />
            </label>

            <dl class="readout">
              <div><dt>Source</dt><dd>{selectedClipFile?.filename ?? 'Missing file'}</dd></div>
              <div><dt>Start</dt><dd>{formatTime(selectedClip.startTime)}</dd></div>
              <div><dt>End</dt><dd>{formatTime(selectedClip.endTime)}</dd></div>
              <div><dt>Length</dt><dd>{formatTime(selectedClipDuration)}</dd></div>
            </dl>

            <div class="button-row">
              <button disabled={selectedClipIndex <= 0 || status === 'exporting'} on:click={() => moveSelectedClip(-1)}>
                Move Left
              </button>
              <button
                disabled={selectedClipIndex === -1 || selectedClipIndex >= clips.length - 1 || status === 'exporting'}
                on:click={() => moveSelectedClip(1)}
              >
                Move Right
              </button>
            </div>
          {:else}
            <p class="muted">Select a clip on the video line.</p>
          {/if}
        </section>

        <section class="panel">
          <div class="panel-title">
            <div>
              <h2>Sources</h2>
              <span>{files.length} files</span>
            </div>
            <button disabled={status === 'exporting'} on:click={resetProject}>Clear</button>
          </div>

          <div class="file-list">
            {#each files as file, index}
              <button
                class:active={index === activeIndex}
                disabled={status === 'exporting'}
                on:click={() => selectFile(index)}
              >
                <span>{file.filename}</span>
                <small>{formatTime(file.duration)}</small>
              </button>
            {/each}
          </div>
        </section>

        <section class="panel">
          <div class="panel-title">
            <div>
              <h2>Output</h2>
              <span>{formatTime(totalDuration)}</span>
            </div>
          </div>

          <label class="field">
            <span>Mode</span>
            <select bind:value={exportMode} disabled={status === 'exporting'}>
              <option value="fast">Fast</option>
              <option value="accurate">Accurate</option>
            </select>
          </label>

          <div class="output-path">
            <span>File</span>
            <strong>{outputPath ?? 'Default output path'}</strong>
          </div>

          <div class="button-row">
            <button disabled={status === 'exporting'} on:click={chooseOutputPath}>Save As</button>
            <button class="primary" disabled={!canExport} on:click={runExport}>
              {status === 'exporting' ? 'Exporting...' : 'Export Connected'}
            </button>
          </div>
        </section>

        {#if status === 'exporting' || exportResult}
          <section class="panel export-status" aria-live="polite">
            {#if status === 'exporting'}
              <strong>Exporting...</strong>
              <span>Building the connected WebM.</span>
            {:else if exportResult}
              <strong>Export complete.</strong>
              <span>{exportResult.outputPath}</span>
              <span>{formatTime(exportResult.duration)}</span>
              <div class="button-row">
                <button on:click={openExportedFile}>Open File</button>
                <button on:click={openExportFolder}>Open Folder</button>
              </div>
            {/if}
          </section>
        {/if}
      </aside>
    </div>
  {/if}
</main>
