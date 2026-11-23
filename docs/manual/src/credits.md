# Credits

This plugin is small, and it stands on other people's work. Thank you to everyone below: the people who wrote the code we build on, and the people who measured, published and explained what the plugin plays.

## Code we use

| Project | What it does here | Licence |
| --- | --- | --- |
| [OxiGuitar's plugin SDK](https://github.com/mateusrdp/oxiguitar) (`oxiguitar-plugin`, and `oxiguitar-host` and `oxiguitar-plugin-test` for the tests) | The plugin interface, and OxiGuitar's own host to test the plugin against | MIT OR Apache-2.0 |
| [Rust](https://www.rust-lang.org/) and its standard library | Everything | MIT OR Apache-2.0 |

The models themselves (`src/intonation.rs`, `src/true_temperament.rs`) are our own code, with no other library.

## Research and published data

| Source | What we took |
| --- | --- |
| G. U. Varieschi and C. M. Gower, [*Intonation and compensation of fretted string instruments*](https://arxiv.org/abs/0906.0127), American Journal of Physics 78, 47 (2010) | The model of the stretched, stiff string |
| M. B. Anderson and R. G. Beausoleil, [*Classical guitar intonation and compensation: the well-tempered guitar*](https://pubmed.ncbi.nlm.nih.gov/39058223/), Journal of the Acoustical Society of America 156, 683 (2024) | A second model to check ours against, and the nylon guitar |
| Anders Thidell and [True Temperament](https://truetemperament.com/) | Thidell Formula 1's published offsets. True Temperament and Thidell Formula 1 are theirs; this plugin only plays the targets they publish and has nothing to do with the company. |
| [D'Addario](https://www.daddario.com/)'s string tension charts | Gauges and tensions of the four guitars |

## Tools

| Tool | Use |
| --- | --- |
| [mdBook](https://github.com/rust-lang/mdBook) | Builds this manual |
| [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) | Checks the licences of everything we build with |

## Thanks

To the late Anders Thidell, whose curved frets started all this, and to Paul Guy, who helped turn them into something every guitarist can play. To the physicists who publish how a guitar actually behaves. And to everyone who tries this plugin and tells us how it sounds to them.
