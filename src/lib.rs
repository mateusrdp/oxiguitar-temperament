// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! OxiGuitar's temperament plugin (plugin interface 1.16;
//! mateusrdp/oxiguitar#294, docs/design.md): a fretted track's notes play
//! at the pitches a real guitar, or a True Temperament neck, would give
//! them, chosen per track in the Track tab.
//!
//! - `intonation` is the real guitar: string stretching and stiffness,
//!   the saddle compensated at the 12th fret.
//! - `strings` holds the guitars it models: nylon, steel, electric, bass.
//! - `true_temperament` is True Temperament's frets, in Thidell Formula 1.
//! - `settings` is a track's choice, `section` shows it in the Track tab
//!   as "Temperament" (`ui/temperament.slint`), and `perform` plays the
//!   notes by it.

use oxiguitar_plugin::{Error, Host, Perform, Performer, PlayedNote, Plugin, Ui};

pub mod intonation;
pub mod perform;
pub mod section;
pub mod settings;
pub mod strings;
pub mod true_temperament;

/// The Track tab's section's id in the manifest: a view and a track
/// section.
pub const SECTION: &str = "temperament";

/// The plugin.
#[derive(Debug)]
pub struct TemperamentPlugin {
    section: section::Section,
}

impl Plugin for TemperamentPlugin {
    fn start(host: Host) -> Result<TemperamentPlugin, Error> {
        Ok(TemperamentPlugin {
            section: section::Section::new(host),
        })
    }

    fn performer(&self) -> Option<&dyn Performer> {
        Some(self)
    }

    fn ui(&self) -> Option<&dyn Ui> {
        Some(&self.section)
    }
}

impl Performer for TemperamentPlugin {
    fn perform(&self, track: &Perform) -> Result<Vec<PlayedNote>, Error> {
        Ok(perform::perform(track))
    }
}

oxiguitar_plugin::export_plugin!(TemperamentPlugin);

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = include_str!("../manifest.json");

    #[test]
    fn the_manifest_is_valid_for_the_plugin() {
        let problems =
            oxiguitar_plugin::check::<TemperamentPlugin>(MANIFEST, env!("CARGO_PKG_VERSION"));
        assert!(problems.is_empty(), "{}", problems.join("; "));
    }
}
