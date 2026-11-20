# Architecture and strategy

OxiGuitar asks the plugin about each track as it gets the song ready to play or export. The plugin gives the same notes back, each with its pitch moved by a small whole number of cents.

For the realistic mode, the plugin models a string with two effects. Pressing the string to the fret stretches it and raises its tension. A stiff string also plays sharper the shorter it is. Each string's saddle is set back until its 12th fret is an exact octave, as most guitars are set up. What is left over is the guitar's own error. The guitars are four plausible examples with real string gauges: nylon, steel, electric and bass. They are not any one instrument.

True Temperament uses Anders Thidell's Formula 1: each note moved by −4 to +4 cents, with the open strings tuned to match.

`docs/design.md` in the repository has the sources and the numbers.
