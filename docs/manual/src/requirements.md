# Requirements and assumptions

## Requirements

- OxiGuitar 0.23.0 or later (plugin interface 1.16). See [the OxiGuitar manual](https://mateusrdp.github.io/oxiguitar/).
- A sound plugin that plays each string on its own pitch. The MIDI plugin does this.
- Windows, macOS or Linux.

## Assumptions

- The guitar is set up as most are, with the 12th fret an exact octave.
- Whole cents are enough. Rounding moves a note by at most half a cent, far below what anyone hears.
- A slide's passing frets are left alone. Each would carry a slightly different error, a cent or two.
