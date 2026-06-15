export type AppStatus =
  | 'idle'
  | 'loading_file'
  | 'file_ready'
  | 'invalid_selection'
  | 'ready_to_export'
  | 'exporting'
  | 'export_done'
  | 'error';

export type CutMode = 'fast' | 'accurate';

export type OpenFileResult = {
  path: string;
  name: string;
  sizeBytes: number;
};

export type MediaInfo = {
  path: string;
  filename: string;
  duration: number;
  sizeBytes: number;
  width: number | null;
  height: number | null;
  videoCodec: string | null;
  audioCodec: string | null;
  fps: number | null;
};

export type CutState = {
  startTime: number | null;
  endTime: number | null;
  duration: number | null;
};

export type CutRequest = {
  inputPath: string;
  outputPath?: string | null;
  startTime: number;
  endTime: number;
  mode: CutMode;
};

export type ExportResult = {
  outputPath: string;
  duration: number;
  sizeBytes: number;
  mode: CutMode;
};

export type AppError = {
  code: string;
  message: string;
  detail?: string | null;
};
