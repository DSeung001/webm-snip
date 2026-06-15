export function formatTime(value: number | null | undefined): string {
  if (value === null || value === undefined || Number.isNaN(value)) {
    return '--:--:--.---';
  }

  const safeValue = Math.max(0, value);
  const hours = Math.floor(safeValue / 3600);
  const minutes = Math.floor((safeValue % 3600) / 60);
  const seconds = Math.floor(safeValue % 60);
  const millis = Math.round((safeValue - Math.floor(safeValue)) * 1000);

  return `${hours.toString().padStart(2, '0')}:${minutes
    .toString()
    .padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${millis
    .toString()
    .padStart(3, '0')}`;
}

export function clampTime(value: number, duration: number): number {
  return Math.min(Math.max(value, 0), Math.max(duration, 0));
}

/** Browser presentation time -> FFmpeg/file timestamp. */
export function toMediaTime(presentationTime: number, videoStartTime: number): number {
  return presentationTime + videoStartTime;
}

/** FFmpeg/file timestamp -> browser presentation time. */
export function toPresentationTime(mediaTime: number, videoStartTime: number): number {
  return Math.max(0, mediaTime - videoStartTime);
}
