import { writable, readable } from 'svelte/store';

export let foodsNormalized = writable([]);
export let dailyTotals = writable({}); 
export let logId = writable(null); 
export const today = readable(new Date(), () => {}); 