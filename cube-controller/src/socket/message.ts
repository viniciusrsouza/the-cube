//! Message types and structs
//!
//! This module contains the message types and structs used by the server and client.
//!
//! # Summary
//! The first byte of every message is the message type. The message type is used to determine
//! how to parse the rest of the message. The message types are defined in the `message_types`
//! module.
//!
//! # Message Types
//! The message types are defined in the `message_types` module. The message types are:
//! - `HANDSHAKE`: The first message sent by the client to the server. The client sends the
//!    server the client's name. The server responds with the server's name.
//! - `SYNC`: The server sends this message to the client to synchronize the client's state
//!   with the server's state. The client does not respond to this message.
//! - `TRANSFORM`: The client sends this message to the server indicating that some change
//!  has been made to the client's state. The server does not respond to this message.
//!
//! ## Transform Message
//! The `TRANSFORM` message type is used to indicate that some change has been made to the
//! client's state. The `TRANSFORM` message type is followed by a transform type byte. The
//! transform type byte is used to determine how to parse the rest of the message. The
//! transform types are defined in the `transform` module.
//!
//! ### Transform Types
//! - `ROTATE`: The client has rotated the object.
//! - `TRANSLATE`: The client has translated the object.
//! - `SCALE`: The client has scaled the object.

enum MessageType {
	HANDSHAKE = 0,
	SYNC = 1,
	TRANSFORM = 2
}

enum TransformType {
	ROTATE = 0,
	TRANSLATE = 1,
	SCALE = 2
}

type Transform = {
	type: MessageType.TRANSFORM;
	transform: Rotate | Translate | Scale;
};

type Rotate = {
	type: TransformType.ROTATE;
	data: [number, number, number];
};

type Translate = {
	type: TransformType.TRANSLATE;
	data: [number, number, number];
};

type Scale = {
	type: TransformType.SCALE;
	data: [number, number, number];
};

type Handshake = {
	type: MessageType.HANDSHAKE;
	name: string;
};

type Sync = {
	type: MessageType.SYNC;
	transform: Transform;
};

type Message = Handshake | Sync | Transform;

function vec3ToBytes(vec: [number, number, number]): Uint8Array {
	const bytes = new Uint8Array(3 * 4);
	const view = new DataView(bytes.buffer, 0, 12);
	view.setFloat32(0, vec[0], true);
	view.setFloat32(4, vec[1], true);
	view.setFloat32(8, vec[2], true);
	return bytes;
}

function vec3FromBytes(bytes: Uint8Array): [number, number, number] {
	const vec: [number, number, number] = [0, 0, 0];
	const view = new DataView(bytes.buffer, bytes.byteOffset, 12);
	vec[0] = view.getFloat32(0, true);
	vec[1] = view.getFloat32(4, true);
	vec[2] = view.getFloat32(8, true);
	return vec;
}

function transformToBytes(transform: Transform): Uint8Array {
	const transformBytes = new Uint8Array(1 + 1 + 3 * 8);
	transformBytes[0] = MessageType.TRANSFORM;
	transformBytes[1] = transform.transform.type;
	const dataBytes = vec3ToBytes(transform.transform.data);
	transformBytes.set(dataBytes, 2);
	return transformBytes;
}

function transformFromBytes(bytes: Uint8Array): Transform {
	const transformType = bytes[0];
	const data = vec3FromBytes(bytes.subarray(1));
	let transform: Rotate | Translate | Scale;
	switch (transformType) {
		case TransformType.ROTATE:
		case TransformType.TRANSLATE:
		case TransformType.SCALE:
			transform = { type: transformType, data };
			break;
		default:
			throw new Error(`Invalid transform type: ${transformType}`);
	}
	return { type: MessageType.TRANSFORM, transform };
}

function handshakeToBytes(handshake: Handshake): Uint8Array {
	const nameBytes = new TextEncoder().encode(handshake.name);
	const handshakeBytes = new Uint8Array(1 + nameBytes.length);
	handshakeBytes[0] = MessageType.HANDSHAKE;
	handshakeBytes.set(nameBytes, 1);
	return handshakeBytes;
}

function handshakeFromBytes(bytes: Uint8Array): Handshake {
	const name = new TextDecoder().decode(bytes.subarray(1));
	return { type: MessageType.HANDSHAKE, name };
}

function syncToBytes(sync: Sync): Uint8Array {
	const transformBytes = transformToBytes(sync.transform);
	const syncBytes = new Uint8Array(1 + transformBytes.length);
	syncBytes[0] = MessageType.SYNC;
	syncBytes.set(transformBytes, 1);
	return syncBytes;
}

function syncFromBytes(bytes: Uint8Array): Sync {
	const transform = transformFromBytes(bytes.subarray(1));
	return { type: MessageType.SYNC, transform };
}

function messageToBytes(message: Message): Uint8Array {
	const type = message.type;
	switch (type) {
		case MessageType.HANDSHAKE:
			return handshakeToBytes(message);
		case MessageType.SYNC:
			return syncToBytes(message);
		case MessageType.TRANSFORM:
			return transformToBytes(message);
		default:
			throw new Error(`Invalid message type: ${type}`);
	}
}

function messageFromBytes(bytes: Uint8Array): Message {
	const messageType = bytes[0];
	switch (messageType) {
		case MessageType.HANDSHAKE:
			return handshakeFromBytes(bytes.subarray(1));
		case MessageType.SYNC:
			return syncFromBytes(bytes.subarray(1));
		case MessageType.TRANSFORM:
			return transformFromBytes(bytes.subarray(1));
		default:
			throw new Error(`Invalid message type: ${messageType}`);
	}
}

export { MessageType, TransformType, messageToBytes, messageFromBytes };
export type { Message, Handshake, Sync, Transform, Rotate, Translate, Scale };
