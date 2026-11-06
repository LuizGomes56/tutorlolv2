//! Observed host stats over game time: the web app's `history/charts.rs`.
//!
//! Every chunk is read once (sequentially, cached by sequence number) and reduced to one
//! sample per frame. Each metric is drawn as a step line, split where capture is missing.
//! Choosing a sample seeks the replay to its game time.
use crate::{
    components::{
        slider::slider,
        ui::{self, duration},
    },
    replay::{ReplaySource, cache::CachedChunk},
    theme,
};
use gpui::{
    Bounds, Context, EventEmitter, IntoElement, ParentElement, PathBuilder, Pixels, Render,
    SharedString, Styled, Task, Window, canvas, div, point, prelude::*, px, rgb,
};
use std::collections::BTreeMap;
use tutorlolv2::riot::RiotRealtime;
use tutorlolv2_protocol::ChunkInfo;

pub const LABELS: [&str; 19] = [
    "Level",
    "Attack damage",
    "Ability power",
    "Armor",
    "Magic resistance",
    "Max health",
    "Attack speed (attacks/s)",
    "Flat armor penetration",
    "Flat magic penetration",
    "Armor penetration (%)",
    "Magic penetration (%)",
    "Crit chance (%)",
    "Crit damage (%)",
    "Kills",
    "Deaths",
    "Assists",
    "CS",
    "Occupied item slots",
    "Game time",
];

/// Charts drawn (the last label, game time, is only the x axis).
const CHARTS: usize = 18;

#[derive(Clone, Debug, PartialEq)]
pub struct Sample {
    pub time: f32,
    pub captured: i64,
    pub seq: u32,
    pub values: [Option<f32>; 19],
}

/// `PlayerStats::base100`, also available without the calculation engine.
pub const fn penetration_percent(raw: f32) -> f32 {
    if raw.is_finite() {
        (1.0 - raw).clamp(0.0, 1.0) * 100.0
    } else {
        f32::NAN
    }
}

/// One sample of the host (the active player), or no values if the host is not found.
pub fn extract(frame: &RiotRealtime<'_>, captured: i64, seq: u32) -> Sample {
    let active = &frame.active_player;
    let host = frame
        .all_players
        .iter()
        .find(|p| !active.riot_id.is_empty() && p.riot_id == active.riot_id);

    let mut values = [None; 19];

    if let Some(host) = host {
        let s = &active.champion_stats;

        let raw = [
            active.level.get() as f32,
            s.attack_damage,
            s.ability_power,
            s.armor,
            s.magic_resist,
            s.max_health,
            s.attack_speed,
            s.armor_penetration_flat,
            s.magic_penetration_flat,
            penetration_percent(s.armor_penetration_percent),
            penetration_percent(s.magic_penetration_percent),
            s.crit_chance,
            s.crit_damage,
            host.scores.kills as f32,
            host.scores.deaths as f32,
            host.scores.assists as f32,
            host.scores.creep_score as f32,
            host.items
                .iter()
                .filter(|i| tutorlolv2::ItemId::from_riot_id(i.item_id).is_some())
                .count() as f32,
            frame.game_data.game_time,
        ];

        for (out, value) in values.iter_mut().zip(raw) {
            *out = value.is_finite().then_some(value)
        }
    }

    Sample {
        time: frame.game_data.game_time,
        captured,
        seq,
        values,
    }
}

/// Sorted by game time; the latest capture wins equal game times.
pub fn normalize(mut points: Vec<Sample>) -> Vec<Sample> {
    points.retain(|p| p.time.is_finite());
    points.sort_by(|a, b| {
        a.time
            .total_cmp(&b.time)
            .then(a.captured.cmp(&b.captured))
            .then(a.seq.cmp(&b.seq))
    });

    let mut out: Vec<Sample> = vec![];

    for point in points {
        if out.last().is_some_and(|p| p.time == point.time) {
            out.pop();
        }

        out.push(point)
    }

    out
}

/// Runs of `(time, value)` without missing values or gaps longer than two seconds.
pub fn segments(points: &[Sample], metric: usize) -> Vec<Vec<(f32, f32)>> {
    let mut result = vec![];
    let mut current = vec![];
    let mut previous = None;

    for point in points {
        if let Some(value) = point.values[metric] {
            if previous.is_some_and(|t: f32| point.time - t > 2.) && !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }

            current.push((point.time, value));
            previous = Some(point.time);
        } else {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }

            previous = None;
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

/// A sample was chosen: seek the replay to this game time.
pub struct Seek(pub f32);

pub struct Charts {
    source: ReplaySource,
    samples: Vec<Sample>,
    status: Option<SharedString>,
    selected: usize,
    /// Samples of each chunk already read, with the metadata they were read for.
    cache: BTreeMap<u32, (ChunkInfo, Vec<Sample>)>,
    _load: Option<Task<()>>,
}

impl EventEmitter<Seek> for Charts {}

impl Charts {
    pub fn new(source: ReplaySource, chunks: Vec<ChunkInfo>, cx: &mut Context<Self>) -> Self {
        let mut charts = Self {
            source,
            samples: Vec::new(),
            status: None,
            selected: 0,
            cache: BTreeMap::new(),
            _load: None,
        };

        charts.load(chunks, cx);
        charts
    }

    /// Reads the chunks not read yet (or changed), then rebuilds the samples.
    pub fn load(&mut self, mut chunks: Vec<ChunkInfo>, cx: &mut Context<Self>) {
        chunks.sort_by(|a, b| a.first_game_time.total_cmp(&b.first_game_time));
        self.status = Some("Reading observed stats…".into());
        cx.notify();

        self._load = Some(cx.spawn(async move |this, cx| {
            let mut failures = 0;

            for metadata in &chunks {
                let Ok(load) = this.update(cx, |this, cx| {
                    let cached = this
                        .cache
                        .get(&metadata.seq)
                        .is_some_and(|(old, _)| old == metadata);

                    (!cached).then(|| this.source.load(metadata.seq, cx))
                }) else {
                    return;
                };

                let Some(load) = load else { continue };
                let metadata = metadata.clone();

                let points = match load.await {
                    Ok(bytes) => {
                        let format = metadata.frames_format;
                        let info = metadata.clone();

                        cx.background_spawn(async move {
                            let chunk = CachedChunk::decode(bytes, format)?;
                            chunk.validate(&info)?;

                            Ok::<_, String>(
                                chunk
                                    .frames()
                                    .iter()
                                    .map(|(captured, frame)| extract(frame, *captured, info.seq))
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .await
                        .ok()
                    }
                    Err(_) => None,
                };

                let Some(points) = points.filter(|points| !points.is_empty()) else {
                    failures += 1;
                    continue;
                };

                let _ = this.update(cx, |this, _| {
                    this.cache.insert(metadata.seq, (metadata, points));
                });
            }

            let _ = this.update(cx, |this, cx| {
                let wanted = chunks.iter().map(|chunk| chunk.seq).collect::<Vec<_>>();
                let all = this
                    .cache
                    .iter()
                    .filter(|(seq, _)| wanted.contains(seq))
                    .flat_map(|(_, (_, points))| points.iter().cloned())
                    .collect();

                this.samples = normalize(all);
                this.selected = this.selected.min(this.samples.len().saturating_sub(1));
                this.status = Some(
                    match failures {
                        0 => "Observed input only. Gaps indicate missing capture; viewer item \
                              simulations do not affect these values."
                            .to_owned(),
                        n => format!(
                            "Observed stats incomplete: {n} chunks unavailable. Refresh details \
                             to retry."
                        ),
                    }
                    .into(),
                );
                cx.notify();
            });
        }));
    }

    fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected = index.min(self.samples.len().saturating_sub(1));

        if let Some(sample) = self.samples.get(self.selected) {
            cx.emit(Seek(sample.time));
        }

        cx.notify();
    }
}

/// One metric as a step line over game time, scaled to its maximum.
fn chart(samples: &[Sample], metric: usize, end: f32, selected: Option<f32>) -> impl IntoElement {
    let maximum = samples
        .iter()
        .filter_map(|point| point.values[metric])
        .fold(1f32, f32::max);

    let segments = segments(samples, metric);

    let plot = canvas(
        |_, _, _| {},
        move |bounds: Bounds<Pixels>, _, window, _| {
            let x = |time: f32| bounds.left() + bounds.size.width * (time / end);
            let y = |value: f32| bounds.top() + bounds.size.height * (0.95 - value / maximum * 0.9);

            for segment in &segments {
                let mut path = PathBuilder::stroke(px(2.));

                for (index, &(time, value)) in segment.iter().enumerate() {
                    match index {
                        0 => path.move_to(point(x(time), y(value))),
                        _ => {
                            // `H{x}V{y}`: horizontal, then vertical.
                            let previous = segment[index - 1].1;
                            path.line_to(point(x(time), y(previous)));
                            path.line_to(point(x(time), y(value)));
                        }
                    }
                }

                if let Ok(path) = path.build() {
                    window.paint_path(path, rgb(0x38bdf8));
                }
            }

            if let Some(time) = selected {
                let mut marker = PathBuilder::stroke(px(1.));
                marker.move_to(point(x(time), bounds.top()));
                marker.line_to(point(x(time), bounds.bottom()));

                if let Ok(marker) = marker.build() {
                    window.paint_path(marker, rgb(theme::STD_500));
                }
            }
        },
    )
    .w_full()
    .h(px(90.));

    div()
        .flex()
        .flex_col()
        .gap_1()
        .min_w(px(240.))
        .flex_1()
        .p_2()
        .rounded_md()
        .border_1()
        .border_color(rgb(theme::STD_800))
        .bg(rgb(theme::STD_950))
        .child(
            div()
                .text_xs()
                .text_color(rgb(theme::STD_400))
                .child(LABELS[metric]),
        )
        .child(plot)
}

impl Render for Charts {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let card = ui::card()
            .child(ui::heading("Observed host stats"))
            .children(self.status.clone().map(ui::muted));

        if self.samples.is_empty() {
            return card.child(ui::muted("No observed stats available."));
        }

        let end = self.samples.last().map_or(1., |p| p.time.max(1.));
        let point = self.samples.get(self.selected).cloned();
        let last = (self.samples.len() - 1) as f32;

        let figures = (0..CHARTS)
            .map(|metric| chart(&self.samples, metric, end, point.as_ref().map(|p| p.time)))
            .collect::<Vec<_>>();

        let rows = LABELS[..CHARTS].iter().enumerate().map(|(index, label)| {
            let value = point
                .as_ref()
                .and_then(|point| point.values[index])
                .map(|value| format!("{value:.2}"))
                .unwrap_or_else(|| "Unavailable".into());

            div()
                .flex()
                .justify_between()
                .py_1()
                .px_2()
                .border_b_1()
                .border_color(rgb(theme::STD_800))
                .text_sm()
                .child(div().text_color(rgb(theme::STD_400)).child(*label))
                .child(div().text_color(rgb(theme::STD_100)).child(value))
        });

        card.child(ui::muted(
            point
                .as_ref()
                .map(|p| format!("Selected sample: game time {}", duration(p.time)))
                .unwrap_or_default(),
        ))
        .child(slider(
            "charts-sample",
            self.selected as f32,
            last.max(1.),
            false,
            &[],
            cx.listener(|this, index: &f32, _, cx| this.select(index.round() as usize, cx)),
        ))
        .child(div().flex().flex_wrap().gap_3().children(figures))
        .child(
            div()
                .flex()
                .flex_col()
                .child(ui::heading("Values at the selected time"))
                .children(rows),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::BINCODE_CONFIG;

    #[test]
    fn charts_split_invalid_metrics_and_missing_capture() {
        let p = |time, value| Sample {
            time,
            captured: 0,
            seq: 0,
            values: [value; 19],
        };
        assert_eq!(
            segments(
                &[
                    p(1., Some(2.)),
                    p(2., None),
                    p(3., Some(3.)),
                    p(8., Some(4.))
                ],
                0
            )
            .len(),
            3
        );
    }

    #[test]
    fn latest_capture_wins_equal_game_time() {
        let mut a = Sample {
            time: 1.,
            captured: 1,
            seq: 1,
            values: [Some(1.); 19],
        };
        let mut b = a.clone();
        b.captured = 2;
        b.values[0] = Some(2.);
        a.seq = 9;
        assert_eq!(normalize(vec![b.clone(), a]), vec![b]);
    }

    #[test]
    fn fixture_host_uses_active_identity_and_base100() {
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/example.bincode"
        ));
        let (mut frame, _): (RiotRealtime<'_>, _) =
            bincode::borrow_decode_from_slice(bytes, BINCODE_CONFIG).unwrap();
        frame.active_player.champion_stats.armor_penetration_percent = 0.7;
        frame.active_player.champion_stats.crit_chance = 25.;
        let sample = extract(&frame, 0, 0);
        assert!((sample.values[9].unwrap() - 30.).abs() < 0.001);
        assert_eq!(sample.values[11], Some(25.));

        let host_index = frame
            .all_players
            .iter()
            .position(|player| player.riot_id == frame.active_player.riot_id)
            .unwrap();
        let expected_cs = frame.all_players[host_index].scores.creep_score;
        frame.all_players.swap(host_index, 1);
        frame.all_players[0].scores.creep_score = 999;
        frame.active_player.champion_stats.attack_damage = f32::NAN;
        frame.active_player.champion_stats.magic_penetration_percent = f32::INFINITY;
        let sample = extract(&frame, 0, 0);
        assert_eq!(sample.values[16], Some(expected_cs as f32));
        assert_eq!(sample.values[1], None);
        assert_eq!(sample.values[10], None);

        frame.active_player.riot_id = "missing";
        assert!(extract(&frame, 0, 0).values.iter().all(Option::is_none));
    }
}
