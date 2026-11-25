# Changelog

All notable changes to the temperament plugin. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed

- **Plugin interface 2.0** (mateusrdp/oxiguitar#346): the manifest asks for `"2.0"` and the SDK is at the commit of mateusrdp/oxiguitar#350 until OxiGuitar 0.24.0 tags it. Interface 2.0 is 1.16 without the plugin UI OxiGuitar drew itself; nothing else changes for this plugin. It needs OxiGuitar 0.24 or later, which no longer loads plugins built for interface 1.

### Changed

- **The Temperament section is drawn in Slint** (mateusrdp/oxiguitar#346, plugin interface 1.16): `ui/temperament.slint`, packed in the plugin file, which OxiGuitar runs with its own widgets, fonts and colours. It shows the same choices as before. The plugin needs OxiGuitar 0.23.0 or later (it already did, since its plugin file is an archive); the README and the manual now say so.

## [0.2.0] - 2026-11-25

### Changed

- **The plugin file is a tar archive** (mateusrdp/oxiguitar#347): `build-oxp.sh` packs `manifest.json` first and the library second (`plugin.so`, `plugin.dll` or `plugin.dylib`), as OxiGuitar 0.23 and later read plugin files, which older OxiGuitars can't load. The SDK is at OxiGuitar v0.23.0 (interface 1.16). `scripts/test-all.py` 1.1.0, as in every repository.

### Added

- **Every test with one command** (mateusrdp/oxiguitar#273): `scripts/test-all.py` builds and runs all of this repository's tests, and tests the plugin against the newest OxiGuitar release, with a progress bar, a summary to paste into an issue and a log to diagnose from. The manual has a Features page naming the tests of every feature (CI checks it) and a Testing page.

### Changed

- **CI outlasts a slow Ubuntu mirror**: it installs packages with `scripts/apt-install.sh`, the same file as in every OxiGuitar repository. apt is stopped only when its downloads have not grown for 2 minutes, then tried again on the next mirror; while they grow, it may take as long as it needs (24 Nov 2026: fixed 5-minute tries failed jobs on mirrors at 80 to 185 kB/s).
- The plugin SDK is now taken from the `v0.21.3` tag of OxiGuitar, the release that has interface 1.12, instead of the commit it was written against (mateusrdp/oxiguitar#319). The code it builds against is the same.
- **The manual explains the ideas behind the plugin** (its overview, with pictures), shows how to use it with screenshots, says in Architecture and strategy what it can't do yet, what is left out on purpose and where help is wanted, and has a Credits page listing every project it uses, with links and thanks. The README has a plain-words introduction and a Credits section.

## [0.1.0] - 2026-11-20

### Added

- **The temperament plugin** (mateusrdp/oxiguitar#294): a fretted track's section **Temperament** in the track dialog chooses how its notes are tuned. The choices are **Off (equal temperament)**; **Realistic guitar**, where fretted notes go sharp as a real guitar's do from string stretching and stiffness, on a nylon, steel, electric or bass guitar set up with its 12th fret in tune; and **True Temperament: Thidell Formula 1**. Bends, slides and vibrato keep their shape. Needs plugin interface 1.12.
