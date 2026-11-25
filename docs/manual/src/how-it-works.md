# Architecture and strategy

## How it fits into OxiGuitar

The plugin is a *performer*: when OxiGuitar gets a song ready to play or to export, it hands the plugin each track's notes, and the plugin gives the same notes back with their pitch moved by a whole number of cents. It never touches the score. The MIDI plugin then plays each string on its own channel with its own pitch bend, so every note of a chord gets its own correction.

- Only fretted tracks (guitars, basses, banjo, sitar) and only notes on a string. Any other track passes through untouched.
- A note's fret is its pitch less its open string's. A capo counts as a fret, since it presses the strings like a finger.
- A note keeps the offset of the fret it starts on for its whole length, so bends, slides and vibrato keep their shape.
- Dead notes have no pitch to move. Natural harmonics sound the open string, so they take the open string's offset.
- The choice is saved per track in the song's `.oxg` file beside it, which OxiGuitar keeps for plugin settings. No score format has a place for it.

## The realistic guitar

The model treats the string as straight segments: nut, finger, fret, saddle. It adds up the two effects from the [overview](index.html#why-a-guitar-doesnt-play-the-notes-exactly), following two published papers:

- G. U. Varieschi and C. M. Gower, *Intonation and compensation of fretted string instruments*, American Journal of Physics 78, 47 (2010), for the stretching and the stiffness.
- M. B. Anderson and R. G. Beausoleil, *Classical guitar intonation and compensation: the well-tempered guitar*, Journal of the Acoustical Society of America 156, 683 (2024).

Each string's saddle is then moved back until its 12th fret is an exact octave, the way a guitar tech sets one up. Whatever error is left is the guitar's own, and that is what you hear.

The four guitars are plausible examples with real string sets and tensions from the makers' charts (D'Addario EJ45 on nylon, EJ16 on steel, EXL110 on electric, EXL170 on bass). They are not copies of any one instrument. `docs/design.md` in the repository has every number, and a table of the result per string and fret.

## True Temperament

The model is a neck whose frets hit Thidell Formula 1's published targets exactly: C +2, C# −4, D +2, Eb −4, E −2, F 0, F# −4, G +4, G# −4, A 0, Bb −4, B −1 cents, with the open strings tuned E −2, A 0, D +2, G +4, B −1, E −1. The fret shapes that get a real neck there are True Temperament's own and aren't modelled: the plugin assumes they do their job.

## Current limitations

- **One finger pressure for every note.** How hard you press changes how sharp a note goes. The model uses one light touch, 0.2 mm past the fret. That number isn't measured; it was chosen to land between the published extremes, and it is the first thing to revisit by ear.
- **Slides ignore the frets they pass.** A slide keeps the offset of the fret it starts on. Each fret it crosses would really have a slightly different error, a cent or two.
- **Whole cents only.** The plugin interface carries pitch in whole cents, so a note can be up to half a cent off the model. Nobody can hear that.
- **Four presets, no custom guitar.** You can't enter your own scale length, string gauges or action yet.
- **No nut compensation.** Most guitars don't have one, so the model doesn't either. Guitars with a compensated nut (such as the Buzz Feiten system) play a little closer to equal temperament near the nut than this model does.
- **Frets above the 24th** take the 24th fret's error.
- **A bass uses the same Thidell table as a guitar.** True Temperament publishes a separate bass preset, which isn't modelled.

## Left out on purpose

- **True Temperament's equal-tempered fret set.** True Temperament also makes necks aimed at plain equal temperament. In this plugin that would sound exactly like **Off**, since the sound plugins already play every note in equal temperament, so it isn't offered (Mateus, 19 November 2026).
- **Perfect temperament, for now.** Just intonation chord by chord is the third option in the issue that started this plugin (mateusrdp/oxiguitar#294). It waits until what it should do is decided: tuned by key, by how each note is spelled, or by each chord. It will be its own task.
- **Changing the score.** Temperament is about the sound only. The plugin never edits notes, tablature or files.
- **Other kinds of instrument.** Keyboards, winds and voices aren't fretted, so the plugin leaves them alone. It is a guitar plugin.

## Where you can help

- **Ears.** The finger-pressure number and the four presets want checking against real guitars. A recording of a guitar with a strobe tuner reading per fret would be gold.
- **Custom setups.** A small dialog for scale length, gauges and action, in place of the presets, is a well-contained job (`src/strings.rs`, `src/section.rs`, `ui/temperament.slint`).
- **Slides that follow the frets.** Needs the note's pitch curve to be split by fret (`src/perform.rs`).
- **Perfect temperament.** If you know just intonation well, join the discussion in mateusrdp/oxiguitar#294 before writing code.

Every change needs its tests and a row on the [Features](features.md) page; the [Testing](testing.md) page says how to run them.
