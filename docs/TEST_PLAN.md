# Test Plan

## File Coverage

- Short WebM under 5 seconds
- Normal WebM from 1 to 5 minutes
- Long WebM over 30 minutes
- Silent WebM
- WebM with audio
- Video-only WebM
- VP8 and VP9 WebM
- Browser-recorded WebM
- OBS-recorded WebM
- MediaRecorder-generated WebM
- Damaged WebM

## Range Coverage

- Cut from 0 seconds
- Cut from the middle
- Cut to the end
- Range shorter than 0.3 seconds
- Start equals end
- Start after end
- End beyond media duration

## Path Coverage

- Korean file paths
- Paths with spaces
- Long paths
- Desktop and Downloads folders
- Permission-denied folders
- Special characters in filenames

## UX Coverage

- Export remains disabled without both points
- Export remains disabled for invalid ranges
- Buttons are disabled while exporting
- Selection playback stops at the end point
- Result file opens after export
- Result folder opens after export
