# Changelog

All notable changes to the temperament plugin. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed

- The plugin SDK is now taken from the `v0.21.3` tag of OxiGuitar, the release that has interface 1.12, instead of the commit it was written against (mateusrdp/oxiguitar#319). The code it builds against is the same.

## [0.1.0] - 2026-11-20

### Added

- **The temperament plugin** (mateusrdp/oxiguitar#294): a fretted track's section **Temperament** in the track dialog chooses how its notes are tuned. The choices are **Off (equal temperament)**; **Realistic guitar**, where fretted notes go sharp as a real guitar's do from string stretching and stiffness, on a nylon, steel, electric or bass guitar set up with its 12th fret in tune; and **True Temperament: Thidell Formula 1**. Bends, slides and vibrato keep their shape. Needs plugin interface 1.12.
