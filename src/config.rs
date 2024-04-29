use evdev::Key;
use serde::Deserialize;

use crate::key::SteamDeckKey;

#[derive(Deserialize)]
pub struct Config {
    pub combo: Vec<Combo>,
    pub mapping: Vec<Mapping>,
}

#[derive(Deserialize)]
pub struct Combo {
    pub keys: Vec<SteamDeckKey>,
    pub launch: String
}

#[derive(Deserialize)]
pub struct Mapping {
    pub from: SteamDeckKey,
    pub to: Vec<Key>
}
