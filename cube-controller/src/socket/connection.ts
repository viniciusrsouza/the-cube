/* eslint-disable @typescript-eslint/no-empty-function */
import { messageFromBytes, type Message, messageToBytes } from "./message";

export class Connection {
  socket: WebSocket | null = null;

  // listeners
  onOpen: () => void = () => {};
  onMessage: (event: Message) => void = () => {};

  constructor(public name: string) {
    this.name = name;
  }

  static getUrl() {
    const host = "192.168.15.8";
    const port = 8080;
    const path = "ws";
    return `ws://${host}:${port}/${path}`;
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
    this.socket = new WebSocket(Connection.getUrl());
    this.socket.addEventListener("open", this.open.bind(this));
    this.socket.addEventListener("message", this.message.bind(this));
  }

  disconnect() {
    this.socket?.close();
  }

  send(message: Message) {
    if (this.socket?.readyState === WebSocket.OPEN) {
      this.socket?.send(messageToBytes(message));
    }
  }
}
