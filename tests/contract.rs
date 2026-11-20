// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! The contract suite: builds this crate as an `.oxp` file and checks it
//! through OxiGuitar's own plugin host, as the app would load it, then has
//! the host ask it to play a track at each temperament. `cargo test` runs
//! it with the rest.

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use oxiguitar_host::api::class::InstrumentClass;
use oxiguitar_host::api::message::{Perform, PerformStaff, PlayedNote};
use oxiguitar_host::{Plugins, SharedSettings, Unheard};

/// This plugin as the `.oxp` file we ship (a debug build), built once.
fn ours() -> &'static Path {
    static BUILT: OnceLock<PathBuf> = OnceLock::new();
    BUILT.get_or_init(|| {
        let crate_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
        // Builds target/oxp/temperament.oxp: the package's name, then the
        // file's.
        oxiguitar_plugin_test::build(crate_folder, env!("CARGO_PKG_NAME"), "temperament", false)
            .expect("the .oxp file builds")
    })
}

#[test]
fn the_plugin_file_keeps_the_contract() {
    let report = oxiguitar_plugin_test::check(ours());
    for passed in &report.passed {
        println!("ok    {passed}");
    }
    assert!(report.ok(), "failed: {:#?}", report.failed);
}

/// Through the host: off, a track plays as written; in Formula 1 its G
/// string's notes move by the published offsets; realistic, a first fret
/// goes sharp; a track of another class is left alone.
#[test]
fn the_host_plays_a_track_at_its_temperament() {
    let scratch = std::env::temp_dir().join(format!(
        "oxiguitar-temperament-contract-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).unwrap();
    std::fs::copy(ours(), scratch.join("temperament.oxp")).unwrap();
    let plugins = Plugins::discover(
        std::slice::from_ref(&scratch),
        SharedSettings::default(),
        "x11",
        Arc::new(Unheard),
    );
    assert_eq!(plugins.performers(), ["oxiguitar.temperament"]);
    let note = |key: u8| PlayedNote {
        staff: 0,
        voice: 0,
        visit: 0,
        beat: 0,
        string: 3,
        key,
        start: 0.0,
        end: 0.5,
        velocity: 95,
        sound: 0,
        harmonic: 0,
        attack: 0,
        flags: 0,
        pitch: Vec::new(),
    };
    let track = |class: InstrumentClass, settings: &str| Perform {
        track: 0,
        class,
        visits: vec![0],
        notes: vec![note(55), note(56), note(67)],
        beats: Vec::new(),
        program: 25,
        staves: vec![PerformStaff {
            tuning: vec![64, 59, 55, 50, 45, 40],
            capo: 0,
        }],
        settings: settings.into(),
    };
    let cents = |class, settings: &str| -> Vec<Vec<i32>> {
        plugins
            .try_perform("oxiguitar.temperament", track(class, settings))
            .unwrap()
            .iter()
            .map(|n| n.pitch.iter().map(|p| p.1).collect())
            .collect()
    };
    let none: Vec<Vec<i32>> = vec![vec![]; 3];
    assert_eq!(cents(InstrumentClass::Fretted, "{}"), none);
    assert_eq!(
        cents(InstrumentClass::Fretted, r#"{"mode": "formula-1"}"#),
        [vec![4], vec![-4], vec![4]]
    );
    let real = cents(InstrumentClass::Fretted, r#"{"mode": "realistic"}"#);
    assert!(
        real[0].is_empty() && real[2].is_empty() && real[1][0] > 0,
        "{real:?}"
    );
    assert_eq!(
        cents(InstrumentClass::Keyboard, r#"{"mode": "formula-1"}"#),
        none
    );
    let _ = std::fs::remove_dir_all(&scratch);
}
