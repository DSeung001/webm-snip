# WebM Snip v0.1 PRD

## Product Sentence

WebM screen recordings에서 필요한 구간만 빠르게 잘라 저장하는 Windows/macOS용 초경량 로컬 컷 편집기.

## Primary Flow

```txt
파일 열기 -> 시작점 지정 -> 끝점 지정 -> 자르기
```

## Required Features

- WebM file open dialog
- Drag and drop path loading
- WebM video preview
- Play and pause
- Seek current playback position
- Mark start and end points
- Show selected duration
- Validate selected range
- Play selected range
- Auto-generate output filename
- Save a new WebM file
- Open result file
- Open result folder
- Friendly error messages

## Non-Goals

- General video editing
- Multi-track editing
- Multiple cut ranges
- Captions, effects, filters, BGM, or transitions
- Cloud or account features
- Project files
