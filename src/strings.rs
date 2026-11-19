// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! The guitars the model plays: a common set of strings on a common
//! instrument for each kind, set up as shops usually set them.
//!
//! Tensions are the makers' published charts (D'Addario's tension chart
//! for EXL110 and EXL170; EJ16 and EJ45 from the packs). The cores of
//! wound strings are not published: they are typical hex-core sizes,
//! about two fifths of the string's diameter, and nylon basses' cores are
//! taken as a solid nylon thread of the same stretch. Heights are common
//! factory set-ups. None of this is one particular guitar: it is a
//! plausible one of its kind, and the errors it gives are of the size
//! measured on real ones (a few cents, more on nylon).

use crate::intonation::{Guitar, StringSpec};

/// Newtons in a pound-force.
const LBF: f64 = 4.448_222;

/// Metres in an inch.
const INCH: f64 = 0.0254;

/// Steel's Young's modulus, in pascals.
const STEEL_E: f64 = 200e9;

/// Nylon's (treble strings), in pascals.
const NYLON_E: f64 = 5e9;

/// How far a finger presses the string below the frets' tops between
/// two frets, in metres: a light touch. Varieschi and Gower press it down
/// to the fingerboard (the frets' height, about 1 mm), which makes low
/// frets 10 to 50 cents sharp, far more than players hear on a set-up
/// guitar; with the string only touching the fret (none) the first fret is
/// 1 to 6 cents sharp. A fifth of a millimetre gives 1 to 11, between the
/// two (docs/design.md).
const PRESS: f64 = 0.0002;

/// A string: its key, tension in pounds and core in inches.
const fn s(key: u8, pounds: f64, core: f64, modulus: f64) -> StringSpec {
    StringSpec {
        key,
        tension: pounds * LBF,
        core: core * INCH,
        modulus,
    }
}

/// A classical guitar: 650 mm, D'Addario EJ45 (normal tension), 3 mm and
/// 4 mm action, a tall nut.
pub const NYLON: Guitar = Guitar {
    scale: 0.650,
    nut: 0.0010,
    action: (0.0030, 0.0040),
    press: PRESS,
    strings: &[
        s(64, 15.6, 0.0285, NYLON_E),
        s(59, 11.6, 0.0322, NYLON_E),
        s(55, 11.9, 0.0403, NYLON_E),
        s(50, 15.0, 0.0200, NYLON_E),
        s(45, 15.4, 0.0230, NYLON_E),
        s(40, 15.5, 0.0260, NYLON_E),
    ],
};

/// A steel-string acoustic: 25.4 in, D'Addario EJ16 (12–53), 2 mm and
/// 2.5 mm action.
pub const STEEL: Guitar = Guitar {
    scale: 25.4 * INCH,
    nut: 0.0006,
    action: (0.0020, 0.0025),
    press: PRESS,
    strings: &[
        s(64, 23.3, 0.012, STEEL_E),
        s(59, 23.3, 0.016, STEEL_E),
        s(55, 30.4, 0.011, STEEL_E),
        s(50, 30.6, 0.014, STEEL_E),
        s(45, 29.6, 0.017, STEEL_E),
        s(40, 26.3, 0.021, STEEL_E),
    ],
};

/// An electric guitar: 25.5 in, D'Addario EXL110 (10–46), 1.6 mm and
/// 2 mm action, medium-jumbo frets.
pub const ELECTRIC: Guitar = Guitar {
    scale: 25.5 * INCH,
    nut: 0.0005,
    action: (0.0016, 0.0020),
    press: PRESS,
    strings: &[
        s(64, 16.2, 0.010, STEEL_E),
        s(59, 15.4, 0.013, STEEL_E),
        s(55, 16.6, 0.017, STEEL_E),
        s(50, 18.4, 0.0125, STEEL_E),
        s(45, 19.0, 0.015, STEEL_E),
        s(40, 16.9, 0.019, STEEL_E),
    ],
};

/// A bass: 34 in, D'Addario EXL170 (45–105), 2.4 mm and 2.8 mm action.
pub const BASS: Guitar = Guitar {
    scale: 34.0 * INCH,
    nut: 0.0006,
    action: (0.0024, 0.0028),
    press: PRESS,
    strings: &[
        s(43, 40.1, 0.016, STEEL_E),
        s(38, 39.6, 0.022, STEEL_E),
        s(33, 43.1, 0.028, STEEL_E),
        s(28, 41.5, 0.034, STEEL_E),
    ],
};

/// The kinds of guitar, as the track section names them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Kind {
    /// From the track's General MIDI program.
    #[default]
    Auto,
    /// [`NYLON`].
    Nylon,
    /// [`STEEL`].
    Steel,
    /// [`ELECTRIC`].
    Electric,
    /// [`BASS`].
    Bass,
}

impl Kind {
    /// Every kind, in the order the section lists them, with its key in
    /// the settings and its name.
    pub const ALL: [(Kind, &'static str, &'static str); 5] = [
        (Kind::Auto, "auto", "From the instrument"),
        (Kind::Nylon, "nylon", "Nylon (classical)"),
        (Kind::Steel, "steel", "Steel (acoustic)"),
        (Kind::Electric, "electric", "Electric"),
        (Kind::Bass, "bass", "Bass"),
    ];

    /// The kind a General MIDI program sounds like: nylon (24), steel (25,
    /// and the banjo and sitar), electric (26 to 31), bass (32 to 39).
    pub fn of_program(program: u8) -> Kind {
        match program {
            24 => Kind::Nylon,
            26..=31 => Kind::Electric,
            32..=39 => Kind::Bass,
            _ => Kind::Steel,
        }
    }

    /// The guitar for this kind, on a track of `program`.
    pub fn guitar(self, program: u8) -> &'static Guitar {
        match self {
            Kind::Auto => Kind::of_program(program).guitar(program),
            Kind::Nylon => &NYLON,
            Kind::Steel => &STEEL,
            Kind::Electric => &ELECTRIC,
            Kind::Bass => &BASS,
        }
    }

    /// Its key in the settings.
    pub fn key(self) -> &'static str {
        Kind::ALL
            .iter()
            .find(|k| k.0 == self)
            .map_or("auto", |k| k.1)
    }

    /// The kind with key `key`; [`Kind::Auto`] for anything else.
    pub fn from_key(key: &str) -> Kind {
        Kind::ALL
            .iter()
            .find(|k| k.1 == key)
            .map_or(Kind::Auto, |k| k.0)
    }
}
