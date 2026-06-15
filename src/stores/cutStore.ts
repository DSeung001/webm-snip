import { writable } from 'svelte/store';
import type { CutState } from '../lib/types';

export const initialCutState: CutState = {
  startTime: null,
  endTime: null,
  duration: null
};

export const cutStore = writable<CutState>(initialCutState);
