# Design: the temperament plugin

From mateusrdp/oxiguitar#294 (Mateus, 19 Nov 2026): "Check the frequencies produced by MIDI. It'd be really cool if they were corrected to match real guitars, with all of its imperfections and compromises." Of the three options the issue names, two are built here: a realistic guitar, and True Temperament. The third, "Perfect Temperament" (just intonation by key and spelling, possibly chord by chord), waits: Mateus is looking into what it should be, and will come back to it as a task of its own.

## Decided by Mateus (19 Nov 2026)

- A plugin of its own, `oxiguitar-temperament`, that plays the notes its own way (`performs`).
- The choice is **per track, saved with the score**: the track dialog's section, kept by the core in `Track::plugins` and the `.oxg` sidecar. No core storage was needed.
- **True Temperament**: Thidell Formula 1. Mateus first asked for both fret sets, selectable, then (19 Nov 2026) **equal temperament dropped**, since it sounds exactly as Off (the sound plugins already play every note in equal temperament) and there is no point offering it.
- The core's interface grew to give a performer what it needs (interface 1.12, mateusrdp/oxiguitar#319): the track's General MIDI program, each staff's tuning and capo, and the track's settings for the plugin. **One `Perform`**, extended in place rather than a second request, since no released plugin performed.

## How it plays

The host asks the plugin about each track when the song is got ready to play or export (`Request::Perform`). The plugin gives back the same notes, each fretted note's pitch curve moved by its offset in whole cents (`src/perform.rs`):

- Only fretted tracks (`InstrumentClass::Fretted`), and only notes on a string. The section shows for fretted tracks alone.
- A note's fret is its key less its string's open key. The capo's fret counts as a fret: a capo presses the strings as a finger does.
- A note keeps the offset of the fret it starts at for its whole length. A bend stays on its fret, so that is right. A slide's own curve carries it from there; the frets it slides past would each have a slightly different error (a cent or two), which is left out.
- Dead notes have no pitch to move. Harmonics sound the open string's partial, so they take the open string's offset: none in the realistic mode (open strings are tuned exactly), the open string's tuning in Formula 1.
- Whole cents are what the interface carries (`PlayedNote::pitch`). Rounding moves a note by at most half a cent, far below what anyone hears (about 5 cents).
- The MIDI plugin gives each string its own channel and pitch bend, so every note of a chord gets its own offset.

## Realistic guitar (`src/intonation.rs`, `src/strings.rs`)

Frets are placed for an ideal string. A real one is not ideal, and fretting it moves its pitch in two ways, after G. U. Varieschi and C. M. Gower, "Intonation and compensation of fretted string instruments", Am. J. Phys. 78, 47 (2010) (arXiv:0906.0127), and M. B. Anderson and R. G. Beausoleil, "Classical guitar intonation and compensation: the well-tempered guitar", J. Acoust. Soc. Am. 156, 683 (2024):

- **Stretching.** Pressed to the fret, and a little towards the fingerboard by the finger, the string is longer than when open, so its tension rises: `T = T0 (1 + kappa Q)`, `Q` the relative stretch and `kappa = E A / T0` (the core's Young's modulus times its cross-section, over the tuned tension). Pitch goes as the square root of tension.
- **Stiffness.** A stiff string vibrates sharp of an ideal one, more so the shorter it is: `f = (1 / 2L) sqrt(T / mu) (1 + B + (1 + pi^2/2) B^2)`, `B = sqrt(E I / (T L^2))`, `I = pi r^4 / 4` (Varieschi and Gower, eq. 10).

The string is straight lines: nut, finger (halfway between the fret behind and the fret), fret, saddle. Its height comes from the action at the 12th fret and the nut's height. Each string's saddle is then set back until its 12th fret is an exact octave, as a luthier sets up nearly every guitar; the nut is left uncompensated. What remains is the guitar's own error. The open string is tuned exactly, so it has none.

**The guitars.** One plausible instrument of each kind, not any one guitar (`src/strings.rs`):

- **Nylon**: 650 mm, D'Addario EJ45, action 3 to 4 mm.
- **Steel**: 25.4 in, D'Addario EJ16 (12 to 53), action 2 to 2.5 mm.
- **Electric**: 25.5 in, D'Addario EXL110 (10 to 46), action 1.6 to 2 mm.
- **Bass**: 34 in, D'Addario EXL170 (45 to 105), action 2.4 to 2.8 mm.

Tensions come from the makers' charts. Wound strings' cores aren't published: they are typical hex-core sizes, about two fifths of the string, and nylon basses' cores are taken as solid nylon of the same stretch. "From the instrument" picks the kind by General MIDI program: 24 nylon, 26 to 31 electric, 32 to 39 bass, anything else steel (25, banjo, sitar).

A track's string takes the set's string made for the nearest pitch. Within two semitones it is that string retuned (drop D, a step down), its tension following the square of the frequency. Further away it stands for a string made for that pitch (a seventh string, a baritone set), at the same tension.

**The finger's press** is the one number that isn't measured. Varieschi and Gower press the string to the fingerboard (about 1 mm below the frets' tops), which makes low frets 10 to 50 cents sharp on a set-up guitar, far more than players hear. With the string only touching the fret, the first fret is 1 to 6 cents sharp. The model takes a light touch, 0.2 mm, which gives 1 to 11 cents: between the two, and a number to revisit by ear.

**What it gives**, in cents from equal temperament (computed by the model):

| Guitar | String | Saddle setback (mm) | Fret 1 | 2 | 3 | 5 | 7 | 12 | 17 | 24 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| nylon | E4 | 1.4 | 1.8 | 0.8 | 0.6 | 0.4 | 0.3 | 0.0 | -0.3 | -1.0 |
| nylon | B3 | 2.3 | 3.2 | 1.4 | 1.0 | 0.7 | 0.4 | 0.0 | -0.5 | -1.6 |
| nylon | G3 | 3.7 | 4.9 | 2.1 | 1.5 | 1.0 | 0.7 | 0.0 | -0.8 | -2.4 |
| nylon | D3 | 0.7 | 1.0 | 0.4 | 0.3 | 0.2 | 0.2 | 0.0 | -0.2 | -0.9 |
| nylon | A2 | 1.0 | 1.3 | 0.6 | 0.4 | 0.3 | 0.2 | 0.0 | -0.3 | -1.1 |
| nylon | E2 | 1.4 | 1.6 | 0.7 | 0.5 | 0.4 | 0.3 | 0.0 | -0.4 | -1.4 |
| steel | E4 | 1.9 | 4.0 | 1.7 | 1.3 | 0.8 | 0.6 | 0.0 | -0.7 | -2.0 |
| steel | B3 | 3.6 | 7.1 | 3.1 | 2.3 | 1.5 | 1.0 | 0.0 | -1.2 | -3.5 |
| steel | G3 | 1.4 | 2.6 | 1.2 | 0.8 | 0.6 | 0.4 | 0.0 | -0.5 | -1.5 |
| steel | D3 | 2.4 | 4.2 | 1.9 | 1.4 | 0.9 | 0.6 | 0.0 | -0.8 | -2.3 |
| steel | A2 | 3.9 | 6.5 | 2.9 | 2.1 | 1.5 | 1.0 | 0.0 | -1.2 | -3.6 |
| steel | E2 | 6.8 | 11.1 | 5.1 | 3.7 | 2.5 | 1.8 | 0.0 | -2.1 | -6.3 |
| electric | E4 | 1.5 | 3.0 | 1.3 | 0.9 | 0.6 | 0.4 | 0.0 | -0.5 | -1.5 |
| electric | B3 | 2.7 | 5.4 | 2.4 | 1.7 | 1.1 | 0.8 | 0.0 | -0.9 | -2.6 |
| electric | G3 | 4.6 | 8.6 | 3.8 | 2.7 | 1.8 | 1.3 | 0.0 | -1.4 | -4.1 |
| electric | D3 | 2.3 | 4.2 | 1.9 | 1.4 | 0.9 | 0.6 | 0.0 | -0.7 | -2.2 |
| electric | A2 | 3.4 | 5.9 | 2.7 | 1.9 | 1.3 | 0.9 | 0.0 | -1.1 | -3.1 |
| electric | E2 | 6.3 | 10.7 | 4.8 | 3.5 | 2.4 | 1.7 | 0.0 | -1.9 | -5.6 |
| bass | G2 | 2.3 | 2.4 | 1.1 | 0.8 | 0.5 | 0.4 | 0.0 | -0.4 | -1.3 |
| bass | D2 | 4.7 | 4.6 | 2.1 | 1.5 | 1.0 | 0.7 | 0.0 | -0.9 | -2.5 |
| bass | A1 | 7.5 | 6.8 | 3.2 | 2.3 | 1.6 | 1.1 | 0.0 | -1.3 | -3.7 |
| bass | E1 | 12.0 | 10.5 | 4.9 | 3.6 | 2.5 | 1.7 | 0.0 | -2.0 | -5.8 |

The saddle setbacks are in the range luthiers use: on an electric, the plain G is the longest of the trebles, the wound D jumps forward, and the low E is set furthest back, as on real guitars.

**Left out:** the nut's own compensation (most guitars have none); pressing harder or softer as the player does (one press for every note); bending stiffness of the wrap; temperature and wear; frets above the 24th (they take the 24th's error).

## True Temperament (`src/true_temperament.rs`)

True Temperament AB (Sweden) makes frets bent under each string, so each note lands on its target with the string's stretching and stiffness already allowed for. The shapes are the maker's own. What they aim at is published, so the model is frets that hit their targets exactly:

- **Thidell Formula 1**, Anders Thidell's temperament for the guitar. Offsets from equal temperament by note: C +2, C# −4, D +2, Eb −4, E −2, F 0, F# −4, G +4, G# −4, A 0, Bb −4, B −1. Open strings in standard tuning: E −2, A 0, D +2, G +4, B −1, E −1 (the first string's open E is 1 cent flat, where a fretted E is 2). Source: Anders Thidell, "Die Wohltemperirte Gitarre / Thidell Formula 1, Technical details" (guyguitars.com/truetemperament, retrieved 19 Nov 2026). Open strings of other tunings take their note's offset.
- **Equal temperament** is not offered: True Temperament makes that fret set too, but every note would be exactly equal-tempered, which is how the sound plugins already play every note, so it would sound as Off. A track saved with the mode `equal` (an early test build) plays as Off.

A bass's notes take the same table.

## Checks

- Unit tests: an ideal string plays equal temperament; every set-up string is in tune at the 12th fret, sharp at the first and within 25 cents everywhere; higher action is sharper; the published Formula 1 table; the settings and the section; perform's rules (open, dead, harmonic, bent notes, other classes).
- The contract suite on the built `.oxp`, and a track played through OxiGuitar's host at each mode (`tests/contract.rs`).
- The private corpus (102 Guitar Pro files, 583,162 notes; not in the repository), with every track at each mode through the host: nothing but pitch curves changed, no note gained or lost. 233,728 notes moved in the realistic mode, by at most 14 cents; 288,442 in Formula 1, by at most 4.

## Later

- Perfect Temperament, when Mateus has decided what it is (#294's third option).
- A slide's offset following the frets it passes.
- Choices for the guitar's set-up (action, gauges, a compensated nut), if the presets prove too few.
