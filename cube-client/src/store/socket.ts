import { Connection } from '$socket/connection';
import type { Message } from '$socket/message';
import { writable } from 'svelte/store';

const messageStore = writable<Message[]>([]);

const conn = new Connection('js-client');

conn.onMessage = (message) => {
	console.log('message', message);
	messageStore.update((messages) => [...messages, message]);
};

export default {
	subscribe: messageStore.subscribe,
	conn
};
