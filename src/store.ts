import { writable, type Writable } from 'svelte/store';
import type { Highlight } from './bindings';

export const todaysHighlight: Writable<Highlight[]> = writable([]);
export const highlights: Writable<Highlight[]> = writable([]);
