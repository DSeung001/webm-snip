import { writable } from 'svelte/store';
import type { ExportResult } from '../lib/types';

export const exportStore = writable<ExportResult | null>(null);
