import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import type { CutRequest, ExportResult, MediaInfo, OpenFileResult } from './types';

export function toVideoSrc(path: string): string {
  return convertFileSrc(path);
}

export function openWebMFile(): Promise<OpenFileResult | null> {
  return invoke('open_webm_file');
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

export function cutWebM(request: CutRequest): Promise<ExportResult> {
  return invoke('cut_webm', { request });
}

export function openFile(path: string): Promise<void> {
  return invoke('open_file_path', { path });
}

export function openFolder(path: string): Promise<void> {
  return invoke('open_folder_path', { path });
}
