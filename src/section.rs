// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! The track dialog's section "Temperament", for fretted tracks: the mode,
//! and for the realistic mode the kind of guitar it models.

use oxiguitar_plugin::serde_json::{Map, Value};
use oxiguitar_plugin::{Error, Row, TabContent, TabEvent, TrackInfo, TrackSection, Widget};

use crate::settings::{Mode, Track};
use crate::strings::Kind;

/// The section.
#[derive(Debug, Default)]
pub struct Section;

fn label(text: impl Into<String>, muted: bool) -> Widget {
    Widget::Label {
        text: text.into(),
        muted,
    }
}

/// What each mode does, under the choice.
fn about(mode: Mode) -> &'static str {
    match mode {
        Mode::Off => "Every note exactly in equal temperament, as written.",
        Mode::Realistic => {
            "Fretted notes go sharp as a real guitar's do: the string stretches when pressed, most near the nut."
        }
        Mode::Formula1 => {
            "True Temperament frets in Thidell Formula 1: purer thirds in the common guitar keys."
        }
    }
}

impl TrackSection for Section {
    fn content(&self, _track: &TrackInfo, settings: &Map<String, Value>) -> TabContent {
        let t = Track::from_map(settings);
        let mut rows = vec![
            Row {
                widgets: vec![
                    label("Tuning", false),
                    Widget::Choice {
                        id: "mode".into(),
                        options: Mode::ALL.iter().map(|m| m.2.to_owned()).collect(),
                        selected: Mode::ALL.iter().position(|m| m.0 == t.mode).unwrap_or(0) as u32,
                        enabled: true,
                    },
                ],
            },
            Row {
                widgets: vec![label(about(t.mode), true)],
            },
        ];
        if t.mode == Mode::Realistic {
            rows.push(Row {
                widgets: vec![
                    label("Guitar", false),
                    Widget::Choice {
                        id: "guitar".into(),
                        options: Kind::ALL.iter().map(|k| k.2.to_owned()).collect(),
                        selected: Kind::ALL.iter().position(|k| k.0 == t.guitar).unwrap_or(0)
                            as u32,
                        enabled: true,
                    },
                ],
            });
        }
        TabContent { rows }
    }

    fn on(
        &self,
        _track: &TrackInfo,
        settings: &mut Map<String, Value>,
        widget: &str,
        event: TabEvent,
    ) -> Result<(), Error> {
        let mut t = Track::from_map(settings);
        match (widget, event) {
            ("mode", TabEvent::Chosen(i)) => match Mode::ALL.get(i as usize) {
                Some(m) => t.mode = m.0,
                None => return Ok(()),
            },
            ("guitar", TabEvent::Chosen(i)) => match Kind::ALL.get(i as usize) {
                Some(k) => t.guitar = k.0,
                None => return Ok(()),
            },
            _ => return Ok(()),
        }
        t.write(settings);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use oxiguitar_plugin::InstrumentClass;

    use super::*;

    fn track() -> TrackInfo {
        TrackInfo {
            index: 0,
            name: "Guitar".into(),
            class: InstrumentClass::Fretted,
        }
    }

    fn choices(content: &TabContent) -> Vec<(String, u32)> {
        content
            .rows
            .iter()
            .flat_map(|r| &r.widgets)
            .filter_map(|w| match w {
                Widget::Choice { id, selected, .. } => Some((id.clone(), *selected)),
                _ => None,
            })
            .collect()
    }

    /// Off by default; the guitar is asked for only in the realistic mode,
    /// and is kept when the mode changes.
    #[test]
    fn the_section_picks_a_mode_and_a_guitar() {
        let s = Section;
        let mut settings = Map::new();
        assert_eq!(
            choices(&s.content(&track(), &settings)),
            [("mode".into(), 0)]
        );
        s.on(&track(), &mut settings, "mode", TabEvent::Chosen(1))
            .unwrap();
        assert_eq!(
            choices(&s.content(&track(), &settings)),
            [("mode".into(), 1), ("guitar".into(), 0)]
        );
        s.on(&track(), &mut settings, "guitar", TabEvent::Chosen(1))
            .unwrap();
        s.on(&track(), &mut settings, "mode", TabEvent::Chosen(2))
            .unwrap();
        assert_eq!(
            choices(&s.content(&track(), &settings)),
            [("mode".into(), 2)]
        );
        assert_eq!(
            Track::from_map(&settings),
            Track {
                mode: Mode::Formula1,
                guitar: Kind::Nylon
            }
        );
        // A pick out of range changes nothing.
        let before = settings.clone();
        s.on(&track(), &mut settings, "mode", TabEvent::Chosen(9))
            .unwrap();
        assert_eq!(settings, before);
    }
}
