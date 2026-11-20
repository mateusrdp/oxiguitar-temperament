// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! How far a real guitar's fretted notes are from equal temperament.
//!
//! Frets are placed for an ideal string: fret `n` at `X (1 − 2^(−n/12))`
//! from the nut, `X` the scale length. A real string is not ideal, and
//! fretting it moves its pitch in two ways (G. U. Varieschi and C. M.
//! Gower, "Intonation and compensation of fretted string instruments",
//! Am. J. Phys. 78, 47 (2010), arXiv:0906.0127; M. B. Anderson and R. G.
//! Beausoleil, "Classical guitar intonation and compensation: the
//! well-tempered guitar", J. Acoust. Soc. Am. 156, 683 (2024)):
//!
//! - **Stretching.** Pressed down to the fret (and, behind it, towards the
//!   fingerboard by the finger), the string is longer than when open, so
//!   its tension rises: `T = T₀ (1 + κ Q)`, `Q` the relative stretch and
//!   `κ = E A / T₀` (Young's modulus times the core's cross-section, over
//!   the tuned tension). The pitch rises with `√T`.
//! - **Stiffness.** A stiff string vibrates as if a little shorter: its
//!   frequency is `f = (1 / 2L) √(T/μ) (1 + B + (1 + π²/2) B²)` with
//!   `B = √(E I / (T L²))`, `I = π r⁴ / 4` for the core of radius `r`
//!   (Varieschi and Gower's eq. 10). Shorter (fretted higher) means
//!   stiffer, so sharper.
//!
//! Both make fretted notes sharp. A luthier sets the saddle back (saddle
//! compensation) until the 12th fret's note is an exact octave of the open
//! string, as nearly every guitar is set up; the nut is left where it is.
//! What remains is the guitar's own error: a few cents sharp near the nut,
//! more on plain and nylon strings and with high action, close to nothing
//! around the 12th fret, slightly flat above it. [`Guitar::tuned`] sets a
//! string up and [`Tuned::cents`] gives its error, from the geometry and
//! the string alone, with straight lines for the string between nut,
//! finger, fret and saddle.

use std::f64::consts::PI;

/// One string of a set, as made for its standard pitch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StringSpec {
    /// The MIDI key it is made to be tuned to.
    pub key: u8,
    /// Its tension at that pitch on the set's scale, in newtons (the
    /// maker's chart).
    pub tension: f64,
    /// The diameter of what carries the tension, in metres: the whole
    /// string when plain, the core when wound.
    pub core: f64,
    /// That core's Young's modulus, in pascals.
    pub modulus: f64,
}

/// A guitar's geometry and its set of strings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Guitar {
    /// The scale length, nut to (uncompensated) saddle, in metres.
    pub scale: f64,
    /// The string's height over the first fret's top at the nut, in
    /// metres: how much the nut holds it above the frets' plane.
    pub nut: f64,
    /// The string's height over the 12th fret's top, the action, on the
    /// highest string and on the lowest, in metres; strings between take
    /// a share by their place.
    pub action: (f64, f64),
    /// How far the finger presses the string below the frets' tops,
    /// halfway between two frets, in metres (Varieschi and Gower take
    /// the frets' height).
    pub press: f64,
    /// The strings, highest first.
    pub strings: &'static [StringSpec],
}

/// A string on a guitar, tuned and set up: what [`StringSpec`] is as
/// tuned to a key, at its height, with its saddle compensated.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuned {
    scale: f64,
    nut: f64,
    action: f64,
    press: f64,
    /// `κ`, the core's stretching stiffness over the tension.
    kappa: f64,
    /// `E I / T₀`, in square metres.
    bending: f64,
    /// How far the saddle sits behind the scale length, in metres.
    setback: f64,
}

/// The highest fret the model answers for; above it a note is the 24th
/// fret's error (no guitar has frets there to stretch over).
pub const FRETS: u8 = 24;

/// The relative distance from the nut of fret `n` (0 the nut, 12 the
/// half), as frets are placed.
fn fret_at(n: f64) -> f64 {
    1.0 - 2f64.powf(-n / 12.0)
}

/// The length of the path through `points`.
fn path(points: &[(f64, f64)]) -> f64 {
    points
        .windows(2)
        .map(|w| (w[1].0 - w[0].0).hypot(w[1].1 - w[0].1))
        .sum()
}

impl Tuned {
    /// Its saddle's height over the frets' plane, from the action at the
    /// 12th fret: the string is straight from the nut to the saddle.
    fn saddle(&self, length: f64) -> f64 {
        let twelfth = self.scale * fret_at(12.0);
        self.nut + (self.action - self.nut) * length / twelfth
    }

    /// The pitch of fret `n` over the open string's, less equal
    /// temperament's, in cents, with the saddle `setback` behind the
    /// scale length.
    fn error_with(&self, n: u8, setback: f64) -> f64 {
        if n == 0 {
            return 0.0;
        }
        let n = f64::from(n);
        let length = self.scale + setback;
        let saddle = self.saddle(length);
        let open = path(&[(0.0, self.nut), (length, saddle)]);
        let fret = self.scale * fret_at(n);
        let finger = self.scale * (fret_at(n - 1.0) + fret_at(n)) / 2.0;
        let pressed = path(&[
            (0.0, self.nut),
            (finger, -self.press),
            (fret, 0.0),
            (length, saddle),
        ]);
        let stretch = (pressed - open) / open;
        let tension = 1.0 + self.kappa * stretch;
        let vibrating = path(&[(fret, 0.0), (length, saddle)]);
        let stiff = |l: f64, t: f64| {
            let b = (self.bending / (t * l * l)).sqrt();
            1.0 + b + (1.0 + PI * PI / 2.0) * b * b
        };
        let ratio =
            open / vibrating * tension.sqrt() * stiff(vibrating, tension) / stiff(open, 1.0);
        1200.0 * ratio.log2() - 100.0 * n
    }

    /// The error of fret `n` (above [`FRETS`], the last fret's), in cents,
    /// on this string as set up.
    pub fn cents(&self, n: u8) -> f64 {
        self.error_with(n.min(FRETS), self.setback)
    }

    /// How far its saddle is set back, in metres.
    pub fn setback(&self) -> f64 {
        self.setback
    }
}

impl Guitar {
    /// The string of the set made for the pitch nearest `key`.
    fn nearest(&self, key: u8) -> Option<&StringSpec> {
        self.strings
            .iter()
            .min_by_key(|s| (i32::from(s.key) - i32::from(key)).abs())
    }

    /// String `string` of `strings` (from 0, the highest) tuned to `key`,
    /// set up as a luthier would: the saddle set back until the 12th fret
    /// plays an exact octave.
    ///
    /// The string is the set's string made for the nearest pitch. Within
    /// two semitones of it, it is that string retuned (drop D, a step
    /// down), its tension following the square of the frequency; further,
    /// it stands for a string made for `key` (a seventh string, a baritone
    /// set), at the same tension.
    pub fn tuned(&self, string: usize, strings: usize, key: u8) -> Option<Tuned> {
        let spec = self.nearest(key)?;
        let apart = f64::from(key) - f64::from(spec.key);
        let tension = if apart.abs() <= 2.0 {
            spec.tension * 2f64.powf(apart / 6.0)
        } else {
            spec.tension
        };
        let share = if strings > 1 {
            string.min(strings - 1) as f64 / (strings - 1) as f64
        } else {
            0.0
        };
        let radius = spec.core / 2.0;
        let mut tuned = Tuned {
            scale: self.scale,
            nut: self.nut,
            action: self.action.0 + (self.action.1 - self.action.0) * share,
            press: self.press,
            kappa: spec.modulus * PI * radius * radius / tension,
            bending: spec.modulus * PI * radius.powi(4) / 4.0 / tension,
            setback: 0.0,
        };
        tuned.setback = tuned.compensated();
        Some(tuned)
    }
}

impl Tuned {
    /// The saddle's setback that makes the 12th fret an exact octave: the
    /// error falls as the saddle moves back, so halving finds it.
    fn compensated(&self) -> f64 {
        let (mut low, mut high) = (0.0, self.scale * 0.05);
        if self.error_with(12, low) <= 0.0 {
            return low;
        }
        for _ in 0..60 {
            let mid = (low + high) / 2.0;
            if self.error_with(12, mid) > 0.0 {
                low = mid;
            } else {
                high = mid;
            }
        }
        (low + high) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strings;

    /// The ideal string: no stretch, no stiffness, nothing to compensate.
    #[test]
    fn an_ideal_string_plays_equal_temperament() {
        let ideal = Tuned {
            scale: 0.65,
            nut: 0.0,
            action: 0.0,
            press: 0.0,
            kappa: 0.0,
            bending: 0.0,
            setback: 0.0,
        };
        for n in 0..=FRETS {
            assert!(ideal.error_with(n, 0.0).abs() < 1e-9, "{n}");
        }
    }

    /// Set up, every string plays its 12th fret in tune, is sharp near the
    /// nut, and its saddle sits a few millimetres back, as on real
    /// guitars (Varieschi and Gower's Table II: 14 to 64 cents before
    /// compensation on a nylon string, a few cents after).
    #[test]
    fn a_set_up_guitar_is_in_tune_at_the_octave_and_sharp_near_the_nut() {
        for guitar in [
            &strings::NYLON,
            &strings::STEEL,
            &strings::ELECTRIC,
            &strings::BASS,
        ] {
            let count = guitar.strings.len();
            for (i, s) in guitar.strings.iter().enumerate() {
                let t = guitar.tuned(i, count, s.key).unwrap();
                assert!(t.cents(12).abs() < 0.01, "{:?}", s);
                assert!(
                    t.cents(1) > 0.5 && t.cents(1) < 25.0,
                    "{s:?}: {}",
                    t.cents(1)
                );
                let mm = t.setback() * 1000.0;
                assert!(mm > 0.3 && mm < 15.0, "{s:?}: {mm} mm");
                for n in 1..=FRETS {
                    assert!(t.cents(n).abs() < 25.0, "{s:?} fret {n}: {}", t.cents(n));
                }
            }
        }
    }

    /// Higher action stretches more: more sharpness near the nut.
    #[test]
    fn higher_action_is_sharper() {
        let low = strings::ELECTRIC;
        let high = Guitar {
            action: (low.action.0 * 2.0, low.action.1 * 2.0),
            ..low
        };
        let key = low.strings[2].key;
        let a = low.tuned(2, 6, key).unwrap().cents(1);
        let b = high.tuned(2, 6, key).unwrap().cents(1);
        assert!(b > a, "{a} {b}");
    }
}
