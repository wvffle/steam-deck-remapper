// Many thanks to the following resources:
// https://github.com/obsiwitch/dotfiles/blob/0743b80eceaccd9d7c3695a10244eead04aee976/packages/sdmap/src/bin/sdmap-daemon.rs#L118
// xremap
mod key;
mod axis;
mod config;

use std::{convert::Infallible, fs};

use config::Config;
use enum_map::{enum_map, EnumMap};
use evdev::{uinput::{VirtualDevice, VirtualDeviceBuilder}, AbsoluteAxisType, AttributeSet, EventStream, EventType, InputEvent, InputEventKind, RelativeAxisType};
use color_eyre::{eyre::OptionExt, Result};

use key::SteamDeckKey;
use axis::Axis;
use tokio::process::Command;

struct Daemon {
    event_stream: EventStream,
    output_device: VirtualDevice,
    cache: EnumMap<Axis, i32>,
    key_combo: Vec<SteamDeckKey>,
    pressed_keys: usize,
    config: Config,
}

fn read_config() -> Result<Config> {
    let mut config_dir = dirs::config_dir()
        .ok_or_eyre("Cannot find config directory")?;
    config_dir.push("steam-deck-remapper");

    fs::create_dir_all(&config_dir)?;
    config_dir.push("config.toml");
    let config_file = config_dir;

    if !config_file.exists() {
        fs::write(&config_file, include_bytes!("../config.example.toml"))?;
    }

    let config_content = fs::read_to_string(config_file)?;
    let config = toml::from_str(&config_content)?;
    Ok(config)
}

impl Daemon {
    fn new() -> Result<Self> {
        let config = read_config()?;

        let input_device = evdev::enumerate()
            .find(|(_, device)| device.name() == Some("Steam Deck"))
            .ok_or_eyre("Steam Deck device not found")?
            .1;

        let output_device = VirtualDeviceBuilder::new()?
            .name("Steam Deck Remapper Device")
            .with_keys(&AttributeSet::from_iter(config.mapping.iter().flat_map(|m| m.to.clone())))?
            .with_relative_axes(&AttributeSet::from_iter([
                // Mouse
                RelativeAxisType::REL_X,
                RelativeAxisType::REL_Y,
                // Mouse wheel
                RelativeAxisType::REL_WHEEL,
                RelativeAxisType::REL_HWHEEL
            ]))?
            .build()?;

        Ok(Daemon {
            event_stream: input_device.into_event_stream()?,
            output_device,
            cache: enum_map! { _ => 0 },
            key_combo: vec![],
            pressed_keys: 0,
            config
        })
    }

    async fn main_loop(&mut self) -> Result<Infallible> {
        let mut events = vec![];
        loop {
            let event = self.event_stream.next_event().await?;

            match event.kind() {
                InputEventKind::AbsAxis(axis) => {
                    if let Some(event) = self.handle_abs_axis_event(axis, event.value()) {
                        events.push(event);
                    }
                },
                InputEventKind::Key(key) => {
                    if let Some(event) = self.handle_key_event(key.into(), event.value() == 1) {
                        events.push(event);
                    }
                },
                InputEventKind::Synchronization(_) => {
                    self.output_device.emit(&events)?;
                    events.clear();
                },

                _ => { dbg!(event); }
            }
        }
    }

    fn handle_key_event(&mut self, key: SteamDeckKey, pressed: bool) -> Option<InputEvent> {
        if pressed {
            self.key_combo.push(key.clone());
            self.pressed_keys += 1;

            // Maybe process combos just before removing the item on keyup?
            for combo in &self.config.combo {
                if combo.keys.len() == self.pressed_keys {
                    let matches = self.key_combo.iter()
                        .all(|key| combo.keys.contains(key));

                    if !matches {
                        continue;
                    }

                    println!("Spawning command: {}", combo.launch);
                    tokio::spawn({
                        let args = combo.launch.clone();

                        async move {
                            let cmd: Vec<_> = args.split_whitespace().collect();

                            Command::new(cmd[0])
                                .args(&cmd[1..])
                                .spawn()
                        }
                    });


                    // NOTE: If a combo is found, we don't process any more combos or mappings
                    return None;
                }
            }

            for mapping in &self.config.mapping {
                if mapping.from != key {
                    continue;
                }

                let events: Vec<_> = mapping.to.iter().map(|key|InputEvent::new(EventType::KEY, key.0, 1)).collect();
                let _ = self.output_device.emit(&events);

                // NOTE: If a macro is found, we don't process any mappings
                return None;
            }


        } else {
            for mapping in &self.config.mapping {
                if mapping.from != key {
                    continue;
                }

                let events: Vec<_> = mapping.to.iter().map(|key|InputEvent::new(EventType::KEY, key.0, 0)).rev().collect();
                let _ = self.output_device.emit(&events);
            }

            self.pressed_keys -= 1;
            self.key_combo.retain(|k| k != &key);
        }

        None
    }

    fn handle_abs_axis_event(&mut self, axis: AbsoluteAxisType, value: i32) -> Option<InputEvent> {
        match axis {
            // NOTE: Right Joystick
            AbsoluteAxisType::ABS_RX => None,
            AbsoluteAxisType::ABS_RY => None,

            // NOTE: Left Joystick
            AbsoluteAxisType::ABS_X => None,
            AbsoluteAxisType::ABS_Y => None,

            // NOTE: Left Trackpad
            AbsoluteAxisType::ABS_HAT0X => self.abs_to_rel(axis, value, 0.0001)
                .map(|delta| InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_HWHEEL.0, delta)),
            AbsoluteAxisType::ABS_HAT0Y => self.abs_to_rel(axis, value, 0.0001)
                .map(|delta| InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_WHEEL.0, delta)),

            // NOTE: Right Trackpad
            AbsoluteAxisType::ABS_HAT1X => self.abs_to_rel(axis, value, 0.005)
                .map(|delta| InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_X.0, delta)),
            AbsoluteAxisType::ABS_HAT1Y => self.abs_to_rel(axis, value, -0.005)
                .map(|delta| InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_Y.0, delta)),

            // NOTE: Left Trigger
            AbsoluteAxisType::ABS_HAT2Y  => None,

            // NOTE: Right Trigger
            AbsoluteAxisType::ABS_HAT2X => None,

            _ => None
        }
    }

    fn abs_to_rel (&mut self, axis: AbsoluteAxisType, value: i32, scale: f32) -> Option<i32> {
        let previous_value = self.cache[axis.into()];

        // Ignore return to center and first value
        if value == 0 || previous_value == 0 {
            self.cache[axis.into()] = value;
            return None;
        }

        let delta = ((value - previous_value) as f32 * scale) as i32;

        // Ignore small changes and stack them for a bigger one
        if delta == 0 { return None; }
        self.cache[axis.into()] = value;
        Some(delta)
    }
}

#[tokio::main]
async fn main () -> Result<()> {
    color_eyre::install()?;

    let mut daemon = Daemon::new()?;
    daemon.main_loop().await?;
    Ok(())
}
