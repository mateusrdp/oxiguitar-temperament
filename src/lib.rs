// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! OxiGuitar's temperament plugin (plugin interface 1.12;
//! mateusrdp/oxiguitar#294, docs/design.md): a fretted track's notes play
//! at the pitches a real guitar, or a True Temperament neck, would give
//! them, chosen per track in the track dialog.
//!
//! - `intonation` is the real guitar: string stretching and stiffness,
//!   the saddle compensated at the 12th fret.
//! - `strings` holds the guitars it models: nylon, steel, electric, bass.
//! - `true_temperament` is True Temperament's frets: Thidell Formula 1,
//!   or equal temperament.
//! - `settings` is a track's choice, `section` shows it in the track
//!   dialog as "Temperament", and `perform` plays the notes by it.

use oxiguitar_plugin::{Error, Host, Perform, Performer, PlayedNote, Plugin, TrackSection};

pub mod intonation;
pub mod perform;
pub mod section;
pub mod settings;
pub mod strings;
pub mod true_temperament;

/// The track dialog's section's id in the manifest.
pub const SECTION: &str = "temperament";

/// The plugin.
#[derive(Debug)]
pub struct TemperamentPlugin;

impl Plugin for TemperamentPlugin {
    fn start(_host: Host) -> Result<TemperamentPlugin, Error> {
        Ok(TemperamentPlugin)
    }

    fn performer(&self) -> Option<&dyn Performer> {
        Some(self)
    }

    fn track_section(&self, id: &str) -> Option<&dyn TrackSection> {
        (id == SECTION).then_some(&section::Section as &dyn TrackSection)
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
