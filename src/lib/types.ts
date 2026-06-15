export type AppStatus =
  | 'idle'
  | 'loading'
  | 'ready'
  | 'exporting'
  | 'done'
  | 'error';

export type ExportMode = 'fast' | 'accurate';

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

export type TimelineSegment = {
  inputIndex: number;
  startTime: number;
  endTime: number;
};

export type TimelineExportRequest = {
  inputPaths: string[];
  outputPath?: string | null;
  segments: TimelineSegment[];
  mode: ExportMode;
};

export type ExportResult = {
  outputPath: string;
  duration: number;
  sizeBytes: number;
  mode: ExportMode;
};

export type AppError = {
  code: string;
  message: string;
  detail?: string | null;
};
