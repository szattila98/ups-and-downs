import { writable, type Writable } from 'svelte/store';
import type { Nullable } from 'ts-toolbelt/out/Union/Nullable';
import type { GroupedHighlight } from './bindings';

export const todaysHighlight: Writable<Nullable<GroupedHighlight>> = writable(null);
