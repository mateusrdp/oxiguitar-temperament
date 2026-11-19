// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! A track's temperament, kept per track beside the score (`FILE.oxg`,
//! the track dialog's section, under `oxiguitar.temperament`):
//!
//! ```json
//! {"mode": "realistic", "guitar": "auto"}
//! ```
//!
//! `mode` is `off` (also when missing or unknown), `realistic`,
//! or `formula-1`; `guitar` is the kind of guitar the realistic
//! mode models, `auto` (from the track's instrument), `nylon`, `steel`,
//! `electric` or `bass`. It is kept whatever the mode, so switching back
//! and forth loses nothing.

use oxiguitar_plugin::serde_json::{Map, Value};

use crate::strings::Kind;

/// How a track's fretted notes are tuned.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    /// As written: equal temperament, as the sound plugins play.
    #[default]
    Off,
    /// As a real guitar plays them ([`crate::intonation`]).
    Realistic,
    /// On True Temperament frets in Thidell Formula 1.
    Formula1,
}

impl Mode {
    /// Every mode, in the order the section lists them, with its key in
    /// the settings and its name.
    pub const ALL: [(Mode, &'static str, &'static str); 3] = [
        (Mode::Off, "off", "Off (equal temperament)"),
        (Mode::Realistic, "realistic", "Realistic guitar"),
        (
            Mode::Formula1,
            "formula-1",
            "True Temperament: Thidell Formula 1",
        ),
    ];

    /// Its key in the settings.
    pub fn key(self) -> &'static str {
        Mode::ALL
            .iter()
            .find(|m| m.0 == self)
            .map_or("off", |m| m.1)
    }

    /// The mode with key `key`; [`Mode::Off`] for anything else.
    pub fn from_key(key: &str) -> Mode {
        Mode::ALL
            .iter()
            .find(|m| m.1 == key)
            .map_or(Mode::Off, |m| m.0)
    }
}

/// A track's settings.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Track {
    /// How its notes are tuned.
    pub mode: Mode,
    /// The guitar the realistic mode models.
    pub guitar: Kind,
}

impl Track {
    /// The settings in `map`; what can't be read is the default.
    pub fn from_map(map: &Map<String, Value>) -> Track {
        let text = |key: &str| map.get(key).and_then(Value::as_str).unwrap_or("");
        Track {
            mode: Mode::from_key(text("mode")),
            guitar: Kind::from_key(text("guitar")),
        }
    }

    /// The settings in a JSON object's text (`Perform::settings`); text
    /// that isn't an object is the default.
    pub fn from_text(text: &str) -> Track {
        match oxiguitar_plugin::serde_json::from_str::<Value>(text) {
            Ok(Value::Object(map)) => Track::from_map(&map),
            _ => Track::default(),
        }
    }

    /// Writes these settings into `map`, keeping any other key.
    pub fn write(&self, map: &mut Map<String, Value>) {
        map.insert("mode".into(), self.mode.key().into());
        map.insert("guitar".into(), self.guitar.key().into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_read_back_and_unknowns_are_off() {
        for t in [
            Track::default(),
            Track {
                mode: Mode::Realistic,
                guitar: Kind::Nylon,
            },
            Track {
                mode: Mode::Formula1,
                guitar: Kind::Bass,
            },
        ] {
            let mut map = Map::new();
            t.write(&mut map);
            assert_eq!(Track::from_map(&map), t);
            assert_eq!(Track::from_text(&Value::Object(map).to_string()), t);
        }
        for odd in [
            "",
            "not json",
            "[]",
            r#"{"mode": 3}"#,
            r#"{"mode": "just"}"#,
        ] {
            assert_eq!(Track::from_text(odd), Track::default(), "{odd}");
        }
    }
}
