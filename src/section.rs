// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! The Track tab's section "Temperament", for fretted tracks: the mode,
//! and for the realistic mode the kind of guitar it models. Drawn by
//! `ui/temperament.slint` (plugin interface 1.16); this side sets its
//! properties and changes the track's settings when a choice is made.

use std::sync::Mutex;

use oxiguitar_plugin::serde_json::{Map, Value as Json};
use oxiguitar_plugin::{Error, Host, TrackView, Ui, Value};

use crate::SECTION;
use crate::settings::{Mode, Track};
use crate::strings::Kind;

/// What each mode does, under the choice.
fn about(mode: Mode) -> &'static str {
    match mode {
        Mode::Off => "Every note exactly in equal temperament, as written.",
        Mode::Realistic => {
            "Fretted notes go sharp as a real guitar's do: the string stretches when pressed, most near the nut."
        }
        Mode::Formula1 => {
            "True Temperament frets in Thidell Formula 1: purer thirds in the common guitar keys."
        }
    }
}

/// The section's properties for a track whose settings are `settings`.
pub fn properties(settings: &Map<String, Json>) -> Vec<(String, Value)> {
    let t = Track::from_map(settings);
    let names = |n: &mut dyn Iterator<Item = &'static str>| {
        Value::List(n.map(|s| Value::String(s.into())).collect())
    };
    vec![
        ("modes".into(), names(&mut Mode::ALL.iter().map(|m| m.2))),
        (
            "mode".into(),
            Value::Number(Mode::ALL.iter().position(|m| m.0 == t.mode).unwrap_or(0) as f64),
        ),
        ("about".into(), Value::String(about(t.mode).into())),
        ("realistic".into(), Value::Bool(t.mode == Mode::Realistic)),
        ("guitars".into(), names(&mut Kind::ALL.iter().map(|k| k.2))),
        (
            "guitar".into(),
            Value::Number(Kind::ALL.iter().position(|k| k.0 == t.guitar).unwrap_or(0) as f64),
        ),
    ]
}

/// Changes `settings` for the section's callback `callback` with `args`;
/// false when it changes nothing (an unknown callback, a pick out of
/// range).
pub fn change(settings: &mut Map<String, Json>, callback: &str, args: &[Value]) -> bool {
    let Some(i) = args.first().and_then(|a| match a {
        Value::Number(n) if *n >= 0.0 => Some(*n as usize),
        _ => None,
    }) else {
        return false;
    };
    let mut t = Track::from_map(settings);
    match callback {
        "mode-chosen" => match Mode::ALL.get(i) {
            Some(m) => t.mode = m.0,
            None => return false,
        },
        "guitar-chosen" => match Kind::ALL.get(i) {
            Some(k) => t.guitar = k.0,
            None => return false,
        },
        _ => return false,
    }
    t.write(settings);
    true
}

/// The section's side in the plugin: the track's settings as the Track
/// tab holds them.
#[derive(Debug)]
pub struct Section {
    host: Host,
    settings: Mutex<Map<String, Json>>,
}

impl Section {
    /// The section, talking to OxiGuitar through `host`.
    pub fn new(host: Host) -> Section {
        Section {
            host,
            settings: Mutex::new(Map::new()),
        }
    }
}

impl Ui for Section {
    fn shown(&self, view: &str, track: Option<&TrackView>) -> Result<(), Error> {
        if view != SECTION {
            return Ok(());
        }
        let settings: Map<String, Json> = track
            .and_then(|t| oxiguitar_plugin::serde_json::from_str(&t.settings).ok())
            .unwrap_or_default();
        let properties = properties(&settings);
        *self.settings.lock().unwrap_or_else(|e| e.into_inner()) = settings;
        self.host.set_view(SECTION, properties)
    }

    fn callback(&self, view: &str, callback: &str, args: Vec<Value>) -> Result<(), Error> {
        if view != SECTION {
            return Ok(());
        }
        let settings = {
            let mut settings = self.settings.lock().unwrap_or_else(|e| e.into_inner());
            if !change(&mut settings, callback, &args) {
                return Ok(());
            }
            settings.clone()
        };
        self.host.set_track_settings(SECTION, &settings)?;
        self.host.set_view(SECTION, properties(&settings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn number(properties: &[(String, Value)], name: &str) -> f64 {
        match properties.iter().find(|(n, _)| n == name) {
            Some((_, Value::Number(n))) => *n,
            other => panic!("{name}: {other:?}"),
        }
    }

    fn flag(properties: &[(String, Value)], name: &str) -> bool {
        matches!(
            properties.iter().find(|(n, _)| n == name),
            Some((_, Value::Bool(true)))
        )
    }

    /// Off by default; the guitar is asked for only in the realistic mode,
    /// and is kept when the mode changes.
    #[test]
    fn the_section_picks_a_mode_and_a_guitar() {
        let mut settings = Map::new();
        let p = properties(&settings);
        assert_eq!(number(&p, "mode"), 0.0);
        assert!(!flag(&p, "realistic"));
        assert!(change(&mut settings, "mode-chosen", &[Value::Number(1.0)]));
        let p = properties(&settings);
        assert_eq!((number(&p, "mode"), number(&p, "guitar")), (1.0, 0.0));
        assert!(flag(&p, "realistic"));
        assert!(change(
            &mut settings,
            "guitar-chosen",
            &[Value::Number(1.0)]
        ));
        assert!(change(&mut settings, "mode-chosen", &[Value::Number(2.0)]));
        let p = properties(&settings);
        assert_eq!(number(&p, "mode"), 2.0);
        assert!(!flag(&p, "realistic"));
        assert_eq!(
            Track::from_map(&settings),
            Track {
                mode: Mode::Formula1,
                guitar: Kind::Nylon
            }
        );
        // A pick out of range, or another callback, changes nothing.
        let before = settings.clone();
        assert!(!change(&mut settings, "mode-chosen", &[Value::Number(9.0)]));
        assert!(!change(&mut settings, "other", &[Value::Number(0.0)]));
        assert_eq!(settings, before);
    }

    /// The file compiles as OxiGuitar compiles it, with every property and
    /// callback this side uses.
    #[test]
    fn the_slint_file_has_what_the_section_sets_and_hears() {
        let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui/temperament.slint");
        let compiled = oxiguitar_plugin_test::compile_ui_file(&file).unwrap();
        let c = compiled
            .components
            .iter()
            .find(|c| c.name == "TemperamentSection")
            .unwrap();
        for (name, _) in properties(&Map::new()) {
            assert!(c.properties.contains(&name), "{name}");
        }
        for name in ["mode-chosen", "guitar-chosen"] {
            assert!(c.callbacks.iter().any(|p| *p == name), "{name}");
        }
    }
}
