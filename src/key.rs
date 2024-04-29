use evdev::Key;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub enum SteamDeckKey {
    X, Y, A, B,
    Up, Right, Down, Left,

    LeftTrackpad, RightTrackpad,
    LeftStick, RightStick,

    L1, L2, L3, L4, R1, R2, R3, R4,

    Select, Start, Steam, Dots,

    Unknown
}

impl From<Key> for SteamDeckKey {
    fn from(key: Key) -> Self {
        match key {
            Key::BTN_NORTH => SteamDeckKey::X,
            Key::BTN_WEST => SteamDeckKey::Y,
            Key::BTN_SOUTH => SteamDeckKey::A,
            Key::BTN_EAST => SteamDeckKey::B,
            Key::BTN_DPAD_UP => SteamDeckKey::Up,
            Key::BTN_DPAD_RIGHT => SteamDeckKey::Right,
            Key::BTN_DPAD_DOWN => SteamDeckKey::Down,
            Key::BTN_DPAD_LEFT => SteamDeckKey::Left,
            Key::BTN_THUMB => SteamDeckKey::LeftTrackpad,
            Key::BTN_THUMB2 => SteamDeckKey::RightTrackpad,
            Key::BTN_THUMBL => SteamDeckKey::LeftStick,
            Key::BTN_THUMBR => SteamDeckKey::RightStick,
            Key::BTN_TL => SteamDeckKey::L1,
            Key::BTN_TL2 => SteamDeckKey::L2,
            Key::BTN_TRIGGER_HAPPY1 => SteamDeckKey::L3,
            Key::BTN_TRIGGER_HAPPY3 => SteamDeckKey::L4,
            Key::BTN_TR => SteamDeckKey::R1,
            Key::BTN_TR2 => SteamDeckKey::R2,
            Key::BTN_TRIGGER_HAPPY2 => SteamDeckKey::R3,
            Key::BTN_TRIGGER_HAPPY4 => SteamDeckKey::R4,
            Key::BTN_SELECT => SteamDeckKey::Select,
            Key::BTN_START => SteamDeckKey::Start,
            Key::BTN_MODE => SteamDeckKey::Steam,
            Key::BTN_BASE => SteamDeckKey::Dots,
            _ => SteamDeckKey::Unknown
        }
    }
}
