import { writable } from 'svelte/store';

export const riskMode = writable<'on' | 'off'>('on');
