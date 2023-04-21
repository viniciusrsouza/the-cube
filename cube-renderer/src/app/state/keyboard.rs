use std::collections::HashMap;

#[derive(Debug)]
pub struct Keyboard {
    is_down: HashMap<Key, (bool, u8)>,
}

#[allow(dead_code)]
impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            is_down: HashMap::new(),
        }
    }

    pub fn on_keydown(&mut self, key: Key, modifiers: u8) {
        self.is_down.insert(key, (true, modifiers));
    }

    pub fn on_keyup(&mut self, key: Key) {
        self.is_down.insert(key, (false, 0));
    }

    pub fn is_down(&mut self, key: Key, mods: u8, strict: bool) -> bool {
        let (is_down, key_mods) = self
            .is_down
            .get(&key)
            .or(Some(&(false, 0)))
            .copied()
            .unwrap_or((false, 0));
        let mods_match;
        if strict {
            mods_match = mods == key_mods;
        } else {
            mods_match = mods & key_mods == mods;
        }
        is_down && mods_match
    }

    pub fn is_up(&mut self, key: Key) -> bool {
        !self.is_down(key, 0, false)
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    Escape, Space, Enter, Tab, Backspace, Insert, Delete, 
    Right, Left, Up, Down, PageUp, PageDown, Home, End,
    CapsLock, ScrollLock, NumLock, PrintScreen, Pause,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide, NumpadDecimal,
    NumpadEnter, NumpadEqual,
    LeftShift, LeftControl, LeftAlt, LeftSuper,
    RightShift, RightControl, RightAlt, RightSuper,
    Backslash, Backquote, BracketLeft, BracketRight, Comma, Minus, Period, Quote, Semicolon, Slash,
    Unknown(String),
}

pub fn from_key_code(code: String) -> Key {
    match code.as_str() {
        "KeyA" => Key::A,
        "KeyB" => Key::B,
        "KeyC" => Key::C,
        "KeyD" => Key::D,
        "KeyE" => Key::E,
        "KeyF" => Key::F,
        "KeyG" => Key::G,
        "KeyH" => Key::H,
        "KeyI" => Key::I,
        "KeyJ" => Key::J,
        "KeyK" => Key::K,
        "KeyL" => Key::L,
        "KeyM" => Key::M,
        "KeyN" => Key::N,
        "KeyO" => Key::O,
        "KeyP" => Key::P,
        "KeyQ" => Key::Q,
        "KeyR" => Key::R,
        "KeyS" => Key::S,
        "KeyT" => Key::T,
        "KeyU" => Key::U,
        "KeyV" => Key::V,
        "KeyW" => Key::W,
        "KeyX" => Key::X,
        "KeyY" => Key::Y,
        "KeyZ" => Key::Z,
        "Digit0" => Key::Num0,
        "Digit1" => Key::Num1,
        "Digit2" => Key::Num2,
        "Digit3" => Key::Num3,
        "Digit4" => Key::Num4,
        "Digit5" => Key::Num5,
        "Digit6" => Key::Num6,
        "Digit7" => Key::Num7,
        "Digit8" => Key::Num8,
        "Digit9" => Key::Num9,
        "Escape" => Key::Escape,
        "Space" => Key::Space,
        "Enter" => Key::Enter,
        "Tab" => Key::Tab,
        "Backspace" => Key::Backspace,
        "Insert" => Key::Insert,
        "Delete" => Key::Delete,
        "ArrowRight" => Key::Right,
        "ArrowLeft" => Key::Left,
        "ArrowUp" => Key::Up,
        "ArrowDown" => Key::Down,
        "PageUp" => Key::PageUp,
        "PageDown" => Key::PageDown,
        "Home" => Key::Home,
        "End" => Key::End,
        "CapsLock" => Key::CapsLock,
        "ScrollLock" => Key::ScrollLock,
        "NumLock" => Key::NumLock,
        "PrintScreen" => Key::PrintScreen,
        "Pause" => Key::Pause,
        "F1" => Key::F1,
        "F2" => Key::F2,
        "F3" => Key::F3,
        "F4" => Key::F4,
        "F5" => Key::F5,
        "F6" => Key::F6,
        "F7" => Key::F7,
        "F8" => Key::F8,
        "F9" => Key::F9,
        "F10" => Key::F10,
        "F11" => Key::F11,
        "F12" => Key::F12,
        "Numpad0" => Key::Numpad0,
        "Numpad1" => Key::Numpad1,
        "Numpad2" => Key::Numpad2,
        "Numpad3" => Key::Numpad3,
        "Numpad4" => Key::Numpad4,
        "Numpad5" => Key::Numpad5,
        "Numpad6" => Key::Numpad6,
        "Numpad7" => Key::Numpad7,
        "Numpad8" => Key::Numpad8,
        "Numpad9" => Key::Numpad9,
        "NumpadAdd" => Key::NumpadAdd,
        "NumpadSubtract" => Key::NumpadSubtract,
        "NumpadMultiply" => Key::NumpadMultiply,
        "NumpadDivide" => Key::NumpadDivide,
        "NumpadDecimal" => Key::NumpadDecimal,
        "NumpadEnter" => Key::NumpadEnter,
        "NumpadEqual" => Key::NumpadEqual,
        "ShiftLeft" => Key::LeftShift,
        "ControlLeft" => Key::LeftControl,
        "AltLeft" => Key::LeftAlt,
        "MetaLeft" => Key::LeftSuper,
        "ShiftRight" => Key::RightShift,
        "ControlRight" => Key::RightControl,
        "AltRight" => Key::RightAlt,
        "MetaRight" => Key::RightSuper,
        "Backslash" => Key::Backslash,
        "Backquote" => Key::Backquote,
        "BracketLeft" => Key::BracketLeft,
        "BracketRight" => Key::BracketRight,
        "Comma" => Key::Comma,
        "Minus" => Key::Minus,
        "Period" => Key::Period,
        "Quote" => Key::Quote,
        "Semicolon" => Key::Semicolon,
        "Slash" => Key::Slash,
        _ => Key::Unknown(code),
    }
}

pub mod modifiers {
    pub const SHIFT: u8 = 1 << 0;
    pub const CTRL: u8 = 1 << 1;
    pub const ALT: u8 = 1 << 2;
    pub const META: u8 = 1 << 3;
}
