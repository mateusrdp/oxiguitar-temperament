// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! Playing a track's notes at its temperament (interface 1.12's
//! `Request::Perform`): each fretted note's pitch moved by its string's
//! and fret's offset, in whole cents, on top of whatever bend, slide or
//! vibrato it already has.
//!
//! - Only fretted tracks change (the section shows for no other), and
//!   only notes on a string of a staff with strings.
//! - A note's fret is its key less its string's open key: the capo's
//!   fret counts as a fret, since a capo presses the strings as a finger
//!   does.
//! - Dead notes have no pitch to move. A harmonic sounds the open
//!   string's partial, so it takes the open string's offset (none in the
//!   realistic mode, where open strings are tuned exactly).
//! - A note keeps one offset for its whole length, the one of the fret it
//!   starts at: a bend stays at that fret, and a slide's own curve takes
//!   it on from there.

use oxiguitar_plugin::audio::sound;
use oxiguitar_plugin::{InstrumentClass, Perform, PlayedNote};

use crate::intonation::Tuned;
use crate::settings::{Mode, Track};
use crate::true_temperament;

/// The track's notes at its temperament.
pub fn perform(track: &Perform) -> Vec<PlayedNote> {
    let settings = Track::from_text(&track.settings);
    if settings.mode == Mode::Off || track.class != InstrumentClass::Fretted {
        return track.notes.clone();
    }
    // Each staff's strings, set up once.
    let tuned: Vec<Vec<Option<Tuned>>> = track
        .staves
        .iter()
        .map(|staff| {
            let guitar = settings.guitar.guitar(track.program);
            let count = staff.tuning.len();
            staff
                .tuning
                .iter()
                .enumerate()
                .map(|(i, &key)| guitar.tuned(i, count, key))
                .collect()
        })
        .collect();
    track
        .notes
        .iter()
        .map(|n| {
            let mut n = n.clone();
            if let Some(cents) = offset(track, &tuned, settings.mode, &n).filter(|c| *c != 0) {
                if n.pitch.is_empty() {
                    n.pitch.push((n.start, 0));
                }
                for p in &mut n.pitch {
                    p.1 = p.1.saturating_add(cents);
                }
            }
            n
        })
        .collect()
}

/// The offset of note `n` in cents, if it has one.
fn offset(
    track: &Perform,
    tuned: &[Vec<Option<Tuned>>],
    mode: Mode,
    n: &PlayedNote,
) -> Option<i32> {
    if n.sound == sound::DEAD {
        return None;
    }
    let string = usize::from(n.string).checked_sub(1)?;
    let staff = usize::try_from(n.staff).ok()?;
    let open = *track.staves.get(staff)?.tuning.get(string)?;
    let fret = n.key.checked_sub(open)?;
    let at_open = fret == 0 || n.sound == sound::HARMONIC;
    match mode {
        Mode::Off => None,
        Mode::Realistic if at_open => None,
        Mode::Realistic => {
            let t = tuned.get(staff)?.get(string)?.as_ref()?;
            Some(t.cents(fret).round() as i32)
        }
        Mode::Formula1 if at_open => Some(true_temperament::formula_1_open(open)),
        Mode::Formula1 => Some(true_temperament::formula_1(n.key)),
    }
}

#[cfg(test)]
mod tests {
    use oxiguitar_plugin::PerformStaff;

    use super::*;

    fn note(string: u8, key: u8, pitch: Vec<(f64, i32)>) -> PlayedNote {
        PlayedNote {
            staff: 0,
            voice: 0,
            visit: 0,
            beat: 0,
            string,
            key,
            start: 1.0,
            end: 1.5,
            velocity: 95,
            sound: sound::NORMAL,
            harmonic: 0,
            attack: 0,
            flags: 0,
            pitch,
        }
    }

    fn track(settings: &str, notes: Vec<PlayedNote>) -> Perform {
        Perform {
            track: 0,
            class: InstrumentClass::Fretted,
            visits: vec![0],
            notes,
            beats: Vec::new(),
            program: 27,
            staves: vec![PerformStaff {
                tuning: vec![64, 59, 55, 50, 45, 40],
                capo: 0,
            }],
            settings: settings.into(),
        }
    }

    fn cents(notes: &[PlayedNote]) -> Vec<Vec<i32>> {
        notes
            .iter()
            .map(|n| n.pitch.iter().map(|p| p.1).collect())
            .collect()
    }

    /// Off, unknown settings (a mode this version doesn't know, such as the
    /// retired `equal`) or a track that isn't fretted: every note as given.
    #[test]
    fn off_changes_nothing() {
        let notes = vec![
            note(3, 56, Vec::new()),
            note(1, 64, vec![(1.0, 0), (1.2, 100)]),
        ];
        for s in ["{}", "nope", r#"{"mode": "off"}"#, r#"{"mode": "equal"}"#] {
            assert_eq!(perform(&track(s, notes.clone())), notes, "{s}");
        }
        let mut keys = track(r#"{"mode": "realistic"}"#, notes.clone());
        keys.class = InstrumentClass::Keyboard;
        assert_eq!(perform(&keys), notes);
    }

    /// Formula 1: G string open +4, its first fret (G#) −4; the first
    /// string open −1 and its 12th fret (E) −2; a bend keeps its shape,
    /// moved; a natural harmonic takes its open string's.
    #[test]
    fn formula_1_moves_each_note_by_its_table() {
        let mut harmonic = note(6, 64, Vec::new());
        harmonic.sound = sound::HARMONIC;
        let played = perform(&track(
            r#"{"mode": "formula-1"}"#,
            vec![
                note(3, 55, Vec::new()),
                note(3, 56, Vec::new()),
                note(1, 64, Vec::new()),
                note(1, 76, vec![(1.0, 0), (1.2, 200)]),
                harmonic,
                note(0, 60, Vec::new()),
            ],
        ));
        assert_eq!(
            cents(&played),
            [vec![4], vec![-4], vec![-1], vec![-2, 198], vec![-2], vec![]]
        );
        assert_eq!(played[0].pitch[0].0, 1.0);
    }

    /// Realistic: open strings exact; low frets sharp, the 12th in tune;
    /// dead notes untouched.
    #[test]
    fn realistic_is_sharp_near_the_nut() {
        let mut dead = note(2, 60, Vec::new());
        dead.sound = sound::DEAD;
        let played = perform(&track(
            r#"{"mode": "realistic", "guitar": "nylon"}"#,
            vec![
                note(3, 55, Vec::new()),
                note(3, 56, Vec::new()),
                note(3, 67, Vec::new()),
                dead,
            ],
        ));
        let c = cents(&played);
        assert!(
            c[0].is_empty() && c[2].is_empty() && c[3].is_empty(),
            "{c:?}"
        );
        assert!(c[1][0] > 0, "{c:?}");
    }
}
