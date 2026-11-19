// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! True Temperament frets (True Temperament AB, Sweden): each fret is bent
//! under each string so that its note lands where the fret set's
//! temperament puts it, the string's stretching and stiffness already
//! allowed for. Their exact shapes are the maker's own; what they aim at
//! is published, so a True Temperament neck is modelled as frets that hit
//! their targets exactly:
//!
//! - **Equal temperament**: every note exactly equal-tempered, as the
//!   sound plugins already play every note. On its own this changes
//!   nothing; it is the "perfect guitar" to compare the others with.
//! - **Thidell Formula 1**, Anders Thidell's temperament for the guitar,
//!   with its per-note offsets from equal temperament and its open-string
//!   tuning, as published in "Die Wohltemperirte Gitarre / Thidell
//!   Formula 1, Technical details" (guyguitars.com/truetemperament,
//!   retrieved 19 Nov 2026).

/// Thidell Formula 1's offsets from equal temperament, in cents, by pitch
/// class from C.
pub const FORMULA_1: [i32; 12] = [
    2,  // C
    -4, // C#
    2,  // D
    -4, // Eb
    -2, // E
    0,  // F
    -4, // F#
    4,  // G
    -4, // G#
    0,  // A
    -4, // Bb
    -1, // B
];

/// Thidell Formula 1's open strings in standard tuning, by MIDI key, in
/// cents: where they differ from the note's offset (the first string's E
/// is tuned 1 cent flat, where a fretted E is 2).
const FORMULA_1_OPEN: [(u8, i32); 6] = [(64, -1), (59, -1), (55, 4), (50, 2), (45, 0), (40, -2)];

/// A fretted note at `key` on a Thidell Formula 1 neck, in cents.
pub fn formula_1(key: u8) -> i32 {
    FORMULA_1[usize::from(key % 12)]
}

/// An open string tuned to `key` on a Thidell Formula 1 neck, in cents:
/// the published tuning for standard tuning's strings, the note's offset
/// for any other.
pub fn formula_1_open(key: u8) -> i32 {
    FORMULA_1_OPEN
        .iter()
        .find(|(k, _)| *k == key)
        .map_or_else(|| formula_1(key), |(_, c)| *c)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The published table: open strings E −2, A 0, D +2, G +4, B −1,
    /// E −1; and G is the sharpest note, the black keys −4.
    #[test]
    fn the_published_offsets() {
        let open: Vec<i32> = [40, 45, 50, 55, 59, 64].map(formula_1_open).to_vec();
        assert_eq!(open, [-2, 0, 2, 4, -1, -1]);
        assert_eq!(formula_1(76), -2);
        assert_eq!(formula_1(67), 4);
        for black in [61, 63, 66, 68, 70] {
            assert_eq!(formula_1(black), -4);
        }
        // A bass's open strings are the same notes, octaves down.
        assert_eq!([28, 33, 38, 43].map(formula_1_open), [-2, 0, 2, 4]);
    }
}
