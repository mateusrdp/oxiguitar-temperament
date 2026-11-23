# Changelog

All notable changes to the temperament plugin. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- **Every test with one command** (mateusrdp/oxiguitar#273): `scripts/test-all.py` builds and runs all of this repository's tests, and tests the plugin against the newest OxiGuitar release, with a progress bar, a summary to paste into an issue and a log to diagnose from. The manual has a Features page naming the tests of every feature (CI checks it) and a Testing page.

### Changed

- The plugin SDK is now taken from the `v0.21.3` tag of OxiGuitar, the release that has interface 1.12, instead of the commit it was written against (mateusrdp/oxiguitar#319). The code it builds against is the same.
- **The manual explains the ideas behind the plugin** (its overview, with pictures), shows how to use it with screenshots, says in Architecture and strategy what it can't do yet, what is left out on purpose and where help is wanted, and has a Credits page listing every project it uses, with links and thanks. The README has a plain-words introduction and a Credits section.

## [0.1.0] - 2026-11-20

### Added

- **The temperament plugin** (mateusrdp/oxiguitar#294): a fretted track's section **Temperament** in the track dialog chooses how its notes are tuned. The choices are **Off (equal temperament)**; **Realistic guitar**, where fretted notes go sharp as a real guitar's do from string stretching and stiffness, on a nylon, steel, electric or bass guitar set up with its 12th fret in tune; and **True Temperament: Thidell Formula 1**. Bends, slides and vibrato keep their shape. Needs plugin interface 1.12.
