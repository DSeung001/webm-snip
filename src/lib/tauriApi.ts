import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import type { ExportResult, MediaInfo, OpenFileResult, TimelineExportRequest } from './types';

export function toVideoSrc(path: string): string {
  return convertFileSrc(path);
}

export function openWebMFiles(): Promise<OpenFileResult[]> {
  return invoke('open_webm_files');
}

export function probeMedia(path: string): Promise<MediaInfo> {
  return invoke('probe_media', { path });
}

export function generateDefaultOutputPath(inputPath: string): Promise<string> {
  return invoke('generate_default_output_path', { inputPath });
}

export function chooseSavePath(defaultPath: string): Promise<string | null> {
  return invoke('choose_save_path', { defaultPath });
}

export function exportTimeline(request: TimelineExportRequest): Promise<ExportResult> {
  return invoke('export_timeline', { request });
}

export function openFile(path: string): Promise<void> {
  return invoke('open_file_path', { path });
}

export function openFolder(path: string): Promise<void> {
  return invoke('open_folder_path', { path });
}
