/* eslint-disable @typescript-eslint/no-empty-function */
import { browser } from '$app/environment';
import { messageFromBytes, type Message, messageToBytes } from './message';

export class Connection {
	socket: WebSocket | null = null;

	// listeners
	onOpen: () => void = () => {};
	onMessage: (event: Message) => void = () => {};

	constructor(public name: string) {
		this.name = name;
	}

	private open() {
		this.onOpen?.();
	}

	private message(event: MessageEvent) {
		const data = event.data;
		if (data instanceof Blob) {
			data
				.arrayBuffer()
				.then((buffer) => {
					const buf = new Uint8Array(buffer);
					const message = messageFromBytes(buf);
					this.onMessage?.(message);
				})
				.catch((err) => {
					console.error(err);
				});
		}
	}

	connect() {
		if (!browser) return;
		this.socket = new WebSocket('ws://localhost:8080/ws');
		this.socket.addEventListener('open', this.open.bind(this));
		this.socket.addEventListener('message', this.message.bind(this));
	}

	disconnect() {
		if (!browser) return;
		this.socket?.close();
	}

	send(message: Message) {
		if (!browser) return;
		if (this.socket?.readyState === WebSocket.OPEN) {
			this.socket?.send(messageToBytes(message));
		}
	}
}
