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

mod message_types {
    pub const HANDSHAKE: u8 = 0;
    pub const SYNC: u8 = 1;
    pub const TRANSFORM: u8 = 2;

    pub mod transform {
        pub const ROTATE: u8 = 0;
        pub const TRANSLATE: u8 = 1;
        pub const SCALE: u8 = 2;
    }
}

pub enum Message {
    Handshake(String),
    Sync(Transform),
    Transform(Transform),
}

pub enum Transform {
    Rotate(glm::Vec3),
    Translate(glm::Vec3),
    Scale(glm::Vec3),
}

pub enum MessageError {
    InvalidMessageType,
    InvalidTransformType,
    InvalidMessageLength,
}

pub trait Serializable {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, MessageError>
    where
        Self: Sized;
}

impl Serializable for glm::Vec3 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.x.to_le_bytes());
        bytes.extend_from_slice(&self.y.to_le_bytes());
        bytes.extend_from_slice(&self.z.to_le_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, MessageError>
    where
        Self: Sized,
    {
        if bytes.len() != 12 {
            return Err(MessageError::InvalidMessageLength);
        }
        let x = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let y = f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let z = f32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        Ok(glm::vec3(x, y, z))
    }
}

impl Serializable for Transform {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Transform::Rotate(vec) => {
                bytes.push(message_types::TRANSFORM);
                bytes.push(message_types::transform::ROTATE);
                bytes.extend_from_slice(&vec.to_bytes());
            }
            Transform::Translate(vec) => {
                bytes.push(message_types::TRANSFORM);
                bytes.push(message_types::transform::TRANSLATE);
                bytes.extend_from_slice(&vec.to_bytes());
            }
            Transform::Scale(vec) => {
                bytes.push(message_types::TRANSFORM);
                bytes.push(message_types::transform::SCALE);
                bytes.extend_from_slice(&vec.to_bytes());
            }
        }
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, MessageError>
    where
        Self: Sized,
    {
        if bytes.len() < 2 {
            return Err(MessageError::InvalidMessageLength);
        }
        match bytes[0] {
            message_types::transform::ROTATE => {
                let vec = glm::Vec3::from_bytes(&bytes[2..])?;
                Ok(Transform::Rotate(vec))
            }
            message_types::transform::TRANSLATE => {
                let vec = glm::Vec3::from_bytes(&bytes[2..])?;
                Ok(Transform::Translate(vec))
            }
            message_types::transform::SCALE => {
                let vec = glm::Vec3::from_bytes(&bytes[2..])?;
                Ok(Transform::Scale(vec))
            }
            _ => Err(MessageError::InvalidTransformType),
        }
    }
}

impl Serializable for Message {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Message::Handshake(name) => {
                bytes.push(message_types::HANDSHAKE);
                bytes.extend_from_slice(name.as_bytes());
            }
            Message::Sync(transform) => {
                bytes.push(message_types::SYNC);
                bytes.extend_from_slice(&transform.to_bytes());
            }
            Message::Transform(transform) => {
                bytes.push(message_types::TRANSFORM);
                bytes.extend_from_slice(&transform.to_bytes());
            }
        }
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, MessageError>
    where
        Self: Sized,
    {
        if bytes.len() < 1 {
            return Err(MessageError::InvalidMessageLength);
        }
        match bytes[0] {
            message_types::HANDSHAKE => {
                let name = String::from_utf8(bytes[1..].to_vec()).unwrap();
                Ok(Message::Handshake(name))
            }
            message_types::SYNC => {
                let transform = Transform::from_bytes(&bytes[1..])?;
                Ok(Message::Sync(transform))
            }
            message_types::TRANSFORM => {
                let transform = Transform::from_bytes(&bytes[1..])?;
                Ok(Message::Transform(transform))
            }
            _ => Err(MessageError::InvalidMessageType),
        }
    }
}
