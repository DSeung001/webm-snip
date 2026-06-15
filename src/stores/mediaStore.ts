import { writable } from 'svelte/store';
import type { MediaInfo } from '../lib/types';

export const mediaStore = writable<MediaInfo | null>(null);
