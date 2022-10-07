// Store for storing URLs and syncing them with localStorage
import { writable } from 'svelte/store'

export interface Url {
    id: string;
    url: string;
}

// Get array from localStorage
let lsData: Url[] = JSON.parse(localStorage.getItem('urls'));

export let urlArr = writable<Array<Url>>(lsData || []);

// On Store update, save it to localStorage
urlArr.subscribe(value => {
    localStorage.urls = JSON.stringify(value);
})
