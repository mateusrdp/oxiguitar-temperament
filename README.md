# oxiguitar-temperament

The temperament plugin for [OxiGuitar](https://github.com/mateusrdp/oxiguitar), a free, open-source guitar tablature and score viewer, editor and player for Windows, macOS and Linux. Sound plugins play every note exactly in equal temperament; a real guitar doesn't. With this plugin a fretted track plays its notes at the pitches a real guitar, or a neck with True Temperament frets, would give them (mateusrdp/oxiguitar#294).

In plain words: a synthesiser plays every note at its exact textbook pitch, and a real guitar never does. Pressing a string stretches it, so fretted notes come out a few cents sharp, most near the nut. This plugin makes guitar and bass tracks sound the way a real instrument would, or the way a guitar with True Temperament's curved frets (tuned to Anders Thidell's Formula 1) would. The [manual](https://mateusrdp.github.io/oxiguitar-temperament/) explains equal temperament, just intonation and True Temperament with pictures.

- Plugin file `temperament.oxp`, id `oxiguitar.temperament`, shown as **Temperament**.
- Per track, in the track dialog's section **Temperament** (fretted tracks: guitars, basses, banjo, sitar), kept in the song's `.oxg` file beside it. **Tuning** is one of:
  - **Off (equal temperament)**, the default: every note as written.
  - **Realistic guitar**: fretted notes go sharp as a real guitar's do. Pressing a string to the fret stretches it, and a stiff string plays sharper the shorter it is. The guitar is set up as most are, with the 12th fret an exact octave. The rest is the guitar's own error: 1 to 11 cents sharp at the first fret (most on the low wound strings and the plain G), under 2 cents from the 7th fret to the 12th, a little flat above. **Guitar** picks the kind modelled: from the track's instrument (nylon, steel, electric or bass by its General MIDI program), or one of them. Open strings, natural harmonics and dead notes stay as they are.
  - **True Temperament: Thidell Formula 1**: each fret hits its target exactly, in Anders Thidell's temperament for the guitar: purer thirds in the guitar's common keys. Notes are moved −4 to +4 cents (C +2, C# −4, D +2, Eb −4, E −2, F 0, F# −4, G +4, G# −4, A 0, Bb −4, B −1), and the open strings are tuned E −2, A 0, D +2, G +4, B −1, E −1.
- Bends, slides and vibrato keep their shape: the note's offset is added to its whole pitch curve.
- It changes only the pitches the player plays. The score, the tablature and the saved file stay as written.

The models, their sources and what they leave out are in [docs/design.md](docs/design.md).

## Manual

The manual is at <https://mateusrdp.github.io/oxiguitar-temperament/>. Its source is in [docs/manual](docs/manual). It covers what the plugin is for, how it works, what it needs and how to install it.

## Install

Every [release](https://github.com/mateusrdp/oxiguitar-temperament/releases) holds a zip per system (Linux x86_64 and ARM64, Windows x86_64 and ARM64, macOS), a `.deb` and an `.rpm` for Linux, a per-user Windows installer (no administrator needed), a macOS `.pkg`, and a `SHA256SUMS` file. The [manual's install page](https://mateusrdp.github.io/oxiguitar-temperament/install.html) says which one to use and how to keep it updated. The plugin needs OxiGuitar 0.21.3 or later (plugin interface 1.12).

## Build and test

Rust 1.99 or later from [rustup](https://rustup.rs). The plugin SDK comes from OxiGuitar's repository by its git tag (`v0.21.3`); nothing of OxiGuitar's is on crates.io.

```sh
cargo test          # unit tests (the models against their sources), the contract suite on the built .oxp file, and a track played through OxiGuitar's plugin host
./build-oxp.sh      # dist/temperament-VERSION-SYSTEM-ARCH.oxp
python3 scripts/test-all.py   # every test, as the manual's Testing page says
```

## Credits

Our own models, written from G. U. Varieschi and C. M. Gower's and M. B. Anderson and R. G. Beausoleil's published work on guitar intonation, and Thidell Formula 1's published offsets (Anders Thidell, True Temperament AB). Built on OxiGuitar's plugin SDK. The manual's [Credits](https://mateusrdp.github.io/oxiguitar-temperament/credits.html) page lists every project with its authors and licence. Thank you to all of them.

## Licence

GPL-3.0-or-later. See LICENSE. Contributions are welcome under the same terms; see [CONTRIBUTING.md](CONTRIBUTING.md).
