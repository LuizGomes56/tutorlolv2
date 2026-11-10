#![feature(const_trait_impl, generic_const_exprs, const_index)]
#![allow(incomplete_features)]
mod engine;
mod groups;
use engine::{Dispatch, Enums, Pointers};
use groups::{EnumGroup, PointerGroup};
use new::{model::*, *};
use std::{hint::black_box, time::Instant};
use strum::EnumCount;

#[derive(Clone, Copy, Debug)]
enum Mode {
    Pointers,
    Enums,
    Tuples,
    PointerBatches,
    EnumBatches,
}
impl Mode {
    fn name(self) -> &'static str {
        match self {
            Self::Pointers => "pointers",
            Self::Enums => "enums",
            Self::Tuples => "tuples",
            Self::PointerBatches => "pointer_batches",
            Self::EnumBatches => "enum_batches",
        }
    }
}
const MODES: [Mode; 5] = [
    Mode::Pointers,
    Mode::Enums,
    Mode::Tuples,
    Mode::PointerBatches,
    Mode::EnumBatches,
];

#[inline(always)]
fn raw_champion(mode: Mode, id: ChampionId, ctx: &Ctx, out: &mut [f32]) {
    match mode {
        Mode::Pointers | Mode::PointerBatches => groups::raw(
            ctx,
            &PointerGroup(groups::pointers_champions_closures(id)),
            out,
        ),
        Mode::Enums | Mode::EnumBatches => groups::raw(ctx, &EnumGroup(id.data().closures), out),
        Mode::Tuples => groups::tuple_raw_champions_closures(id, ctx, out),
    }
}
#[inline(always)]
fn ability(
    mode: Mode,
    id: ChampionId,
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    m: Modifiers,
    out: &mut [i32],
) {
    match mode {
        Mode::Pointers => Pointers::<false> { id }.abilities(id, ctx, onhit, m, out),
        Mode::Enums => Enums::<false> { id }.abilities(id, ctx, onhit, m, out),
        Mode::PointerBatches => Pointers::<true> { id }.abilities(id, ctx, onhit, m, out),
        Mode::EnumBatches => Enums::<true> { id }.abilities(id, ctx, onhit, m, out),
        Mode::Tuples => groups::tuple_eval_champions_closures(id, ctx, out, onhit, m),
    }
}
#[inline(always)]
fn item(
    mode: Mode,
    id: ItemId,
    a: AttackType,
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    m: Modifiers,
    out: &mut [i32],
) {
    let any = ChampionId::default();
    match mode {
        Mode::Pointers | Mode::PointerBatches => {
            Pointers::<false> { id: any }.item(id, a, ctx, onhit, m, out)
        }
        Mode::Enums | Mode::EnumBatches => {
            Enums::<false> { id: any }.item(id, a, ctx, onhit, m, out)
        }
        Mode::Tuples => match a {
            AttackType::Melee => groups::tuple_eval_items_melee(id, ctx, out, onhit, m),
            AttackType::Ranged => groups::tuple_eval_items_ranged(id, ctx, out, onhit, m),
        },
    }
}
#[inline(always)]
fn rune(
    mode: Mode,
    id: RuneId,
    a: AttackType,
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    m: Modifiers,
    out: &mut [i32],
) {
    let any = ChampionId::default();
    match mode {
        Mode::Pointers | Mode::PointerBatches => {
            Pointers::<false> { id: any }.rune(id, a, ctx, onhit, m, out)
        }
        Mode::Enums | Mode::EnumBatches => {
            Enums::<false> { id: any }.rune(id, a, ctx, onhit, m, out)
        }
        Mode::Tuples => match a {
            AttackType::Melee => groups::tuple_eval_runes_melee(id, ctx, out, onhit, m),
            AttackType::Ranged => groups::tuple_eval_runes_ranged(id, ctx, out, onhit, m),
        },
    }
}
#[inline(always)]
fn calculator(mode: Mode, game: InputGame<'_>) -> engine::Output {
    let id = game.active_player.data.champion_id;
    match mode {
        Mode::Pointers => engine::calculator(game, &Pointers::<false> { id }),
        Mode::Enums => engine::calculator(game, &Enums::<false> { id }),
        Mode::PointerBatches => engine::calculator(game, &Pointers::<true> { id }),
        Mode::EnumBatches => engine::calculator(game, &Enums::<true> { id }),
        Mode::Tuples => groups::tuple_calculator(game),
    }
}

// Every champion's tuple is actually evaluated during constant evaluation.
const TUPLE_CHECKSUM: u64 = {
    let ctx = new::test::CTX;
    let mut result = 0u64;
    let mut i = 0;
    while i < ChampionId::COUNT {
        let id = ChampionId::from_repr(i as _).unwrap();
        let n = id.number_of_abilities();
        let mut out = [0; 18];
        let mut onhit = RangeDamage::default();
        groups::tuple_eval_champions_closures(
            id,
            &ctx,
            &mut out[..n],
            &mut onhit,
            Modifiers::default(),
        );
        let mut j = 0;
        while j < n {
            result = result.wrapping_add(out[j] as u64);
            j += 1;
        }
        result = result
            .wrapping_add(onhit.minimum_damage as u64)
            .wrapping_add(onhit.maximum_damage as u64);
        i += 1;
    }
    result
};

fn modified() -> Modifiers {
    let mut m = Modifiers::default();
    m.damages.global_mod = 0.87;
    m.damages.physical_mod = 0.61;
    m.damages.magic_mod = 0.73;
    m.damages.true_mod = 1.13;
    m.abilities.q = 1.17;
    m.abilities.w = 0.83;
    m.abilities.e = 1.29;
    m.abilities.r = 0.91;
    m
}
fn validate() {
    let mut checksum = 0u64;
    for ctx in [
        new::test::CTX,
        Ctx {
            ability_power: 600.0,
            attack_damage: 400.0,
            enemy_current_health: 250.0,
            ..new::test::CTX
        },
    ] {
        for i in 0..ChampionId::COUNT {
            let id = ChampionId::from_repr(i as _).unwrap();
            let n = id.number_of_abilities();
            let mut expected = vec![0.0; n];
            raw_champion(Mode::Pointers, id, &ctx, &mut expected);
            for mode in MODES {
                let mut actual = vec![0.0; n];
                raw_champion(mode, id, &ctx, &mut actual);
                assert!(
                    actual
                        .iter()
                        .zip(&expected)
                        .all(|(a, b)| a.to_bits() == b.to_bits()),
                    "raw {id:?} {mode:?}"
                );
                for m in [Modifiers::default(), modified()] {
                    let mut actual = vec![0; n];
                    let mut expected = vec![0; n];
                    let mut ah = RangeDamage::default();
                    let mut eh = RangeDamage::default();
                    ability(Mode::Pointers, id, &ctx, &mut eh, m, &mut expected);
                    ability(mode, id, &ctx, &mut ah, m, &mut actual);
                    assert_eq!(actual, expected, "ability {id:?} {mode:?}");
                    assert_eq!(ah, eh, "onhit {id:?} {mode:?}");
                }
            }
        }
        for i in 0..ItemId::COUNT {
            let id = ItemId::from_repr(i as _).unwrap();
            for a in [AttackType::Melee, AttackType::Ranged] {
                for mode in MODES {
                    let mut actual = [0; 2];
                    let mut expected = [0; 2];
                    let mut ah = RangeDamage::default();
                    let mut eh = RangeDamage::default();
                    item(
                        Mode::Pointers,
                        id,
                        a,
                        &ctx,
                        &mut eh,
                        modified(),
                        &mut expected,
                    );
                    item(mode, id, a, &ctx, &mut ah, modified(), &mut actual);
                    assert_eq!((actual, ah), (expected, eh), "item {id:?} {a:?} {mode:?}");
                }
            }
        }
        for i in 0..RuneId::COUNT {
            let id = RuneId::from_repr(i as _).unwrap();
            for a in [AttackType::Melee, AttackType::Ranged] {
                for mode in MODES {
                    let mut actual = [0; 2];
                    let mut expected = [0; 2];
                    let mut ah = RangeDamage::default();
                    let mut eh = RangeDamage::default();
                    rune(
                        Mode::Pointers,
                        id,
                        a,
                        &ctx,
                        &mut eh,
                        modified(),
                        &mut expected,
                    );
                    rune(mode, id, a, &ctx, &mut ah, modified(), &mut actual);
                    assert_eq!((actual, ah), (expected, eh), "rune {id:?} {a:?} {mode:?}");
                }
            }
        }
    }
    for i in 0..ChampionId::COUNT {
        let id = ChampionId::from_repr(i as _).unwrap();
        let mut out = vec![0; id.number_of_abilities()];
        let mut onhit = RangeDamage::default();
        ability(
            Mode::Tuples,
            id,
            black_box(&new::test::CTX),
            &mut onhit,
            Modifiers::default(),
            &mut out,
        );
        for v in out {
            checksum = checksum.wrapping_add(v as u64);
        }
        checksum = checksum
            .wrapping_add(onhit.minimum_damage as u64)
            .wrapping_add(onhit.maximum_damage as u64);
        let mut game = new::test::GAME;
        game.active_player.data.champion_id = id;
        for game in [
            game,
            InputGame {
                game_map: GameMap::AramMayhem,
                ..game
            },
        ] {
            let reference = new::calculator::calculator(game);
            for mode in MODES {
                let actual = calculator(mode, game);
                assert_eq!(
                    actual.current_player, reference.current_player,
                    "player {id:?} {mode:?}"
                );
                assert_eq!(actual.enemies, reference.enemies, "enemies {id:?} {mode:?}");
                assert_eq!(actual.items_meta, reference.items_meta);
                assert_eq!(actual.runes_meta, reference.runes_meta);
                assert_eq!(actual.tower_damages, reference.tower_damages);
                for r in 0..reference.damages.rows() {
                    assert_eq!(
                        actual.damages.row(r),
                        reference.damages.row(r),
                        "row {id:?} {mode:?} {r}"
                    );
                }
            }
        }
    }
    assert_eq!(
        checksum, TUPLE_CHECKSUM,
        "constant vs runtime tuple results"
    );
    eprintln!(
        "Validated all champion/item/rune groups, both attack types, modifiers/onhit, const evaluation, and all five calculator strategies against the current calculator."
    );
}

fn shuffle(xs: &mut [usize]) {
    let mut s = 0xb7a941d3fe26c805u64;
    for i in (1..xs.len()).rev() {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        xs.swap(i, s as usize % (i + 1));
    }
}
struct Harness {
    ctx: Ctx,
    ids: Vec<ChampionId>,
    games: Vec<InputGame<'static>>,
    items: Vec<ItemId>,
    runes: Vec<RuneId>,
}
impl Harness {
    fn new(order: &str) -> Self {
        let mut indices = (0..ChampionId::COUNT).collect::<Vec<_>>();
        match order {
            "shuffled" => shuffle(&mut indices),
            "neeko" => indices = vec![ChampionId::Neeko as usize],
            "leblanc" => indices = vec![ChampionId::Leblanc as usize],
            _ => {}
        }
        let ids = indices
            .iter()
            .map(|&i| ChampionId::from_repr(i as _).unwrap())
            .collect::<Vec<_>>();
        let games = ids
            .iter()
            .map(|&id| {
                let mut g = new::test::GAME;
                g.active_player.data.champion_id = id;
                g
            })
            .collect();
        let mut result = Self {
            ctx: new::test::CTX,
            ids,
            games,
            items: if order == "neeko" || order == "leblanc" {
                vec![ItemId::NashorsTooth]
            } else {
                ItemId::DAMAGING_IDS.to_vec()
            },
            runes: if order == "neeko" || order == "leblanc" {
                vec![RuneId::Electrocute]
            } else {
                RuneId::DAMAGING_IDS.to_vec()
            },
        };
        if order == "shuffled" {
            let mut order = (0..result.items.len()).collect::<Vec<_>>();
            shuffle(&mut order);
            result.items = order.iter().map(|&i| result.items[i]).collect();
            let mut order = (0..result.runes.len()).collect::<Vec<_>>();
            shuffle(&mut order);
            result.runes = order.iter().map(|&i| result.runes[i]).collect();
        }
        result
    }
}
#[derive(Clone, Copy, Debug)]
enum Work {
    Raw,
    Rows,
    Pairs,
    Calculator,
}
impl Work {
    fn name(self) -> &'static str {
        match self {
            Self::Raw => "raw_groups",
            Self::Rows => "ability_rows",
            Self::Pairs => "item_rune_pairs",
            Self::Calculator => "calculator",
        }
    }
}

#[inline(never)]
fn kernel<const M: u8>(work: Work, h: &Harness) -> u64 {
    let mode = match M {
        0 => Mode::Pointers,
        1 => Mode::Enums,
        2 => Mode::Tuples,
        3 => Mode::PointerBatches,
        4 => Mode::EnumBatches,
        _ => unreachable!(),
    };
    let mut sum = 0u64;
    let ctx = black_box(&h.ctx);
    match work {
        Work::Raw => {
            let mut out = [0.0; 18];
            for &id in black_box(&h.ids) {
                let row = &mut out[..id.number_of_abilities()];
                raw_champion(mode, id, ctx, row);
                for v in black_box(row) {
                    sum = sum.wrapping_add(v.to_bits() as u64);
                }
            }
        }
        Work::Rows => {
            let mut out = [0; 18];
            for &id in black_box(&h.ids) {
                let row = &mut out[..id.number_of_abilities()];
                let mut onhit = RangeDamage::default();
                ability(mode, id, ctx, &mut onhit, Modifiers::default(), row);
                sum = sum
                    .wrapping_add(black_box(row)[0] as u64)
                    .wrapping_add(onhit.minimum_damage as u64);
            }
        }
        Work::Pairs => {
            let mut out = [0; 2];
            for &id in black_box(&h.items) {
                for a in [AttackType::Melee, AttackType::Ranged] {
                    let mut onhit = RangeDamage::default();
                    item(mode, id, a, ctx, &mut onhit, Modifiers::default(), &mut out);
                    sum = sum
                        .wrapping_add(black_box(out)[0] as u64)
                        .wrapping_add(out[1] as u64)
                        .wrapping_add(onhit.minimum_damage as u64);
                }
            }
            for &id in black_box(&h.runes) {
                for a in [AttackType::Melee, AttackType::Ranged] {
                    let mut onhit = RangeDamage::default();
                    rune(mode, id, a, ctx, &mut onhit, Modifiers::default(), &mut out);
                    sum = sum
                        .wrapping_add(black_box(out)[0] as u64)
                        .wrapping_add(out[1] as u64)
                        .wrapping_add(onhit.minimum_damage as u64);
                }
            }
        }
        Work::Calculator => {
            for &game in black_box(&h.games) {
                let result = black_box(calculator(mode, black_box(game)));
                sum = sum.wrapping_add(result.damages.row(L_MSTR).sum() as u64);
            }
        }
    }
    black_box(sum)
}
fn call_kernel(mode: Mode, work: Work, h: &Harness) -> u64 {
    match mode {
        Mode::Pointers => kernel::<0>(work, h),
        Mode::Enums => kernel::<1>(work, h),
        Mode::Tuples => kernel::<2>(work, h),
        Mode::PointerBatches => kernel::<3>(work, h),
        Mode::EnumBatches => kernel::<4>(work, h),
    }
}
fn time_one<const M: u8>(work: Work, h: &Harness, n: usize) -> f64 {
    let start = Instant::now();
    for _ in 0..n {
        black_box(kernel::<M>(work, black_box(h)));
    }
    start.elapsed().as_secs_f64()
}
fn time(mode: Mode, work: Work, h: &Harness, n: usize) -> f64 {
    match mode {
        Mode::Pointers => time_one::<0>(work, h, n),
        Mode::Enums => time_one::<1>(work, h, n),
        Mode::Tuples => time_one::<2>(work, h, n),
        Mode::PointerBatches => time_one::<3>(work, h, n),
        Mode::EnumBatches => time_one::<4>(work, h, n),
    }
}

fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn measure(work: Work, h: &Harness) -> serde_json::Value {
    let modes: &[Mode] = if matches!(work, Work::Raw | Work::Pairs) {
        &MODES[..3]
    } else {
        &MODES
    };
    let expected = call_kernel(Mode::Pointers, work, h);
    for &m in modes {
        assert_eq!(call_kernel(m, work, h), expected);
        for _ in 0..10 {
            black_box(call_kernel(m, work, h));
        }
    }
    let mut n = 1;
    while time(Mode::Pointers, work, h, n) < 0.020 {
        n *= 2;
    }
    let operations = if matches!(work, Work::Pairs) {
        (h.items.len() + h.runes.len()) * 2
    } else {
        h.ids.len()
    };
    let mut samples = vec![vec![]; modes.len()];
    // Rotate and reverse the five strategy orders to balance thermal/time drift.
    for round in 0..21 {
        let mut order = (0..modes.len()).collect::<Vec<_>>();
        order.rotate_left(round % modes.len());
        if round % 2 == 1 {
            order.reverse();
        }
        for k in order {
            let ns = time(modes[k], work, h, n) * 1e9 / (n * operations) as f64;
            samples[k].push(ns);
        }
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    let values=modes.iter().enumerate().map(|(i,m)|{
        let baseline=if matches!(work,Work::Calculator){3}else{0};
        let ratios=samples[i].iter().zip(&samples[baseline]).map(|(x,b)|x/b).collect::<Vec<_>>();
        eprintln!("{} {} {:.2} ns/op; {:.3}x reference",work.name(),m.name(),median(&samples[i]),median(&ratios));
        serde_json::json!({"strategy":m.name(),"median_ns":median(&samples[i]),"ratio_reference":median(&ratios),"samples_ns":samples[i]})
    }).collect::<Vec<_>>();
    serde_json::json!({"workload":work.name(),"operations_per_sweep":operations,"sweeps_per_sample":n,"samples":21,"reference":if matches!(work,Work::Calculator){"pointer_batches"}else{"pointers"},"strategies":values})
}
fn main() {
    validate();
    if std::env::args().any(|a| a == "--validate-only") {
        return;
    }
    let mut results = vec![];
    for order in ["grouped", "shuffled", "neeko", "leblanc"] {
        let h = Harness::new(order);
        eprintln!("Scenario {order}: {} champions", h.ids.len());
        for work in [Work::Raw, Work::Rows, Work::Pairs, Work::Calculator] {
            if matches!(work, Work::Pairs) && (order == "neeko" || order == "leblanc") {
                continue;
            }
            results.push(serde_json::json!({"order":order,"measurement":measure(work,&h)}));
        }
    }
    println!("{}",serde_json::to_string_pretty(&serde_json::json!({"profile":"release opt3, fat LTO, codegen-units=1; engine only","results":results})).unwrap());
}
