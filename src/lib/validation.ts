import type { CutState } from './types';

export const MIN_CUT_DURATION_SECONDS = 0.3;

export function getCutDuration(startTime: number | null, endTime: number | null): number | null {
  if (startTime === null || endTime === null) {
    return null;
  }

  return Math.max(0, endTime - startTime);
}

export function validateCutRange(cut: CutState, mediaDuration: number | null): string | null {
  if (cut.startTime === null || cut.endTime === null) {
    return '시작점과 끝점을 지정해 주세요.';
  }

  if (cut.startTime >= cut.endTime) {
    return '끝점은 시작점보다 뒤에 있어야 합니다.';
  }

  if (cut.endTime - cut.startTime < MIN_CUT_DURATION_SECONDS) {
    return '선택 구간은 최소 0.3초 이상이어야 합니다.';
  }

  if (mediaDuration !== null && cut.endTime > mediaDuration) {
    return '끝점이 영상 길이를 넘을 수 없습니다.';
  }

  return null;
}
