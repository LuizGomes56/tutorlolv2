//! Same-binary, same-compiler comparison. Does not alter library dispatch.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use new::{AbilityId, Ctx, TypeMetadata};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};
use strum::EnumCount;
use tutorlolv2 as new;
use tutorlolv2_stable as old;
type OldFormula = fn(&Ctx) -> f32;
type OldBatch = fn(&Ctx, old::model::Modifiers, &mut old::model::RangeDamage, &mut [i32]);

struct Harness {
    ctx: Ctx,
    formulas_new: Vec<new::Closure>,
    formulas_old: Vec<OldFormula>,
    batches_new: Vec<new::AbilityBatch>,
    batches_old: Vec<OldBatch>,
    groups_new: Vec<&'static [new::Closure]>,
    groups_old: Vec<&'static [OldFormula]>,
    metadata: Vec<&'static [TypeMetadata<AbilityId>]>,
    games_new: Vec<new::model::InputGame<'static>>,
    games_old: Vec<old::model::InputGame<'static>>,
}

fn old_game(id: old::ChampionId) -> old::model::InputGame<'static> {
    use old::{model::*, *};
    const LEVEL: std::num::NonZeroU8 = std::num::NonZeroU8::new(18).unwrap();
    const ENEMIES: [InputMinData<'static, EnemyStats>; 1] = [InputMinData {
        champion_id: ChampionId::Aatrox,
        items: &[ItemId::ForceOfNature],
        item_exceptions: &[],
        stats: None,
        stacks: 0,
        level: LEVEL,
        is_mega_gnar: false,
    }];
    InputGame {
        active_player: InputActivePlayer {
            runes: &[RuneId::Electrocute],
            rune_exceptions: &[],
            abilities: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            current_health: 0.0,
            data: InputMinData {
                champion_id: id,
                items: &[
                    ItemId::NashorsTooth,
                    ItemId::RabadonsDeathcap,
                    ItemId::SorcerersShoes,
                ],
                item_exceptions: &[],
                stats: None,
                stacks: 0,
                level: LEVEL,
                is_mega_gnar: false,
            },
        },
        enemy_players: &ENEMIES,
        dragons: Dragons::default(),
        game_map: GameMap::Unknown,
    }
}

fn shuffle(indices: &mut [usize]) {
    let mut state = 0xb7a941d3fe26c805u64;
    for i in (1..indices.len()).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        indices.swap(i, state as usize % (i + 1));
    }
}

impl Harness {
    fn new(shuffled: bool) -> Self {
        let mut h = Self {
            ctx: new::test::CTX,
            formulas_new: vec![],
            formulas_old: vec![],
            batches_new: vec![],
            batches_old: vec![],
            groups_new: vec![],
            groups_old: vec![],
            metadata: vec![],
            games_new: vec![],
            games_old: vec![],
        };
        let mut order = (0..new::ChampionId::COUNT).collect::<Vec<_>>();
        if shuffled {
            shuffle(&mut order);
        }
        for i in order {
            let id = new::ChampionId::from_repr(i as _).unwrap();
            let a = id.data();
            let b = old::ChampionId::from_repr(i as _).unwrap().data();
            h.formulas_new.extend_from_slice(a.closures);
            h.formulas_old.extend_from_slice(b.closures);
            h.batches_new.push(a.ability_batch);
            h.batches_old.push(b.ability_batch);
            h.groups_new.push(a.closures);
            h.groups_old.push(b.closures);
            h.metadata.push(a.metadata);
            let mut game = new::test::GAME;
            game.active_player.data.champion_id = id;
            h.games_new.push(game);
            h.games_old
                .push(old_game(old::ChampionId::from_repr(i as _).unwrap()));
        }
        // Only damaging item/rune pairs: avoid benchmarking thousands of zero entries.
        for id in new::ItemId::DAMAGING_IDS {
            let a = id.data();
            let b = old::ItemId::from_repr(id as _).unwrap().data();
            h.formulas_new.extend(a.melee);
            h.formulas_new.extend(a.ranged);
            h.formulas_old.extend(b.melee);
            h.formulas_old.extend(b.ranged);
        }
        for id in new::RuneId::DAMAGING_IDS {
            let a = id.data();
            let b = old::RuneId::from_repr(id as _).unwrap().data();
            h.formulas_new.extend(a.melee);
            h.formulas_new.extend(a.ranged);
            h.formulas_old.extend(b.melee);
            h.formulas_old.extend(b.ranged);
        }
        if shuffled {
            let mut order = (0..h.formulas_new.len()).collect::<Vec<_>>();
            shuffle(&mut order);
            h.formulas_new = order.iter().map(|&i| h.formulas_new[i]).collect();
            h.formulas_old = order.iter().map(|&i| h.formulas_old[i]).collect();
        }
        assert!(h.metadata.iter().all(|m| m.len() <= 64));
        h
    }

    fn single(id: new::ChampionId) -> Self {
        let mut h = Self::new(false);
        let index = id as usize;
        macro_rules! keep_one {
            ($($field:ident),*) => { $(h.$field.swap(0, index); h.$field.truncate(1);)* };
        }
        keep_one!(
            batches_new,
            batches_old,
            groups_new,
            groups_old,
            metadata,
            games_new,
            games_old
        );
        h.formulas_new = h.groups_new[0].to_vec();
        h.formulas_old = h.groups_old[0].to_vec();
        // The same ranged damaging item and rune used by the calculator fixtures.
        h.formulas_new
            .extend(new::ItemId::NashorsTooth.data().ranged);
        h.formulas_old
            .extend(old::ItemId::NashorsTooth.data().ranged);
        h.formulas_new
            .extend(new::RuneId::Electrocute.data().ranged);
        h.formulas_old
            .extend(old::RuneId::Electrocute.data().ranged);
        h
    }
}

#[inline(never)]
fn enum_formulas(h: &Harness) -> u64 {
    let mut sum = 0u64;
    let ctx = black_box(&h.ctx);
    for f in black_box(h.formulas_new.as_slice()) {
        sum = sum.wrapping_add(f(ctx).to_bits() as u64);
    }
    black_box(sum)
}
#[inline(never)]
fn pointer_formulas(h: &Harness) -> u64 {
    let mut sum = 0u64;
    let ctx = black_box(&h.ctx);
    for f in black_box(h.formulas_old.as_slice()) {
        sum = sum.wrapping_add(f(ctx).to_bits() as u64);
    }
    black_box(sum)
}
#[inline(never)]
fn enum_batches(h: &Harness) -> u64 {
    let mut sum = 0u64;
    let mut out = [0; 64];
    let ctx = black_box(&h.ctx);
    for (f, metadata) in black_box(&h.batches_new).iter().zip(black_box(&h.metadata)) {
        let mut onhit = new::model::RangeDamage::default();
        let row = &mut out[..metadata.len()];
        f(ctx, new::model::Modifiers::default(), &mut onhit, row);
        sum = sum
            .wrapping_add(black_box(row)[0] as u64)
            .wrapping_add(onhit.minimum_damage as u64);
    }
    black_box(sum)
}
#[inline(never)]
fn pointer_batches(h: &Harness) -> u64 {
    let mut sum = 0u64;
    let mut out = [0; 64];
    let ctx = black_box(&h.ctx);
    for (f, metadata) in black_box(&h.batches_old).iter().zip(black_box(&h.metadata)) {
        let mut onhit = old::model::RangeDamage::default();
        let row = &mut out[..metadata.len()];
        f(ctx, old::model::Modifiers::default(), &mut onhit, row);
        sum = sum
            .wrapping_add(black_box(row)[0] as u64)
            .wrapping_add(onhit.minimum_damage as u64);
    }
    black_box(sum)
}
#[inline(never)]
fn enum_rows(h: &Harness) -> u64 {
    let mut sum = 0u64;
    let mut out = [0; 64];
    let ctx = black_box(&h.ctx);
    for (fs, metadata) in black_box(&h.groups_new).iter().zip(black_box(&h.metadata)) {
        let mut onhit = new::model::RangeDamage::default();
        let row = &mut out[..metadata.len()];
        new::model::DamageTable::eval_abilities(
            ctx,
            &mut onhit,
            new::model::Modifiers::default(),
            metadata,
            fs,
            row,
        );
        sum = sum
            .wrapping_add(black_box(row)[0] as u64)
            .wrapping_add(onhit.minimum_damage as u64);
    }
    black_box(sum)
}
#[inline(never)]
fn pointer_rows(h: &Harness) -> u64 {
    let mut sum = 0u64;
    let mut out = [0; 64];
    let ctx = black_box(&h.ctx);
    for (fs, metadata) in black_box(&h.groups_old).iter().zip(black_box(&h.metadata)) {
        let mut onhit = old::model::RangeDamage::default();
        let row = &mut out[..metadata.len()];
        old::model::DamageTable::eval_abilities(
            ctx,
            &mut onhit,
            old::model::Modifiers::default(),
            metadata,
            fs,
            row,
        );
        sum = sum
            .wrapping_add(black_box(row)[0] as u64)
            .wrapping_add(onhit.minimum_damage as u64);
    }
    black_box(sum)
}
#[inline(never)]
fn enum_calculator(h: &Harness) -> u64 {
    let mut sum = 0u64;
    for &game in black_box(&h.games_new) {
        let output = black_box(new::calculator::calculator(black_box(game)));
        sum = sum.wrapping_add(output.enemy_damages(0).sum() as u64);
    }
    black_box(sum)
}
#[inline(never)]
fn pointer_calculator(h: &Harness) -> u64 {
    let mut sum = 0u64;
    for &game in black_box(&h.games_old) {
        let output = black_box(old::calculator::calculator(black_box(game)));
        sum = sum.wrapping_add(output.enemy_damages(0).sum() as u64);
    }
    black_box(sum)
}

type Kernel = fn(&Harness) -> u64;
fn time(kernel: Kernel, h: &Harness, repeats: usize) -> f64 {
    let start = Instant::now();
    for _ in 0..repeats {
        black_box(kernel(black_box(h)));
    }
    start.elapsed().as_secs_f64()
}
fn percentile(xs: &[f64], p: f64) -> f64 {
    let mut xs = xs.to_vec();
    xs.sort_by(f64::total_cmp);
    xs[((xs.len() - 1) as f64 * p).round() as usize]
}
fn measure(
    label: &str,
    h: &Harness,
    pointer: Kernel,
    en: Kernel,
    operations: usize,
) -> serde_json::Value {
    assert_eq!(pointer(h), en(h), "preflight checksum mismatch: {label}");
    for _ in 0..10 {
        black_box(pointer(h));
        black_box(en(h));
    }
    let mut repeats = 1;
    while time(pointer, h, repeats) < 0.020 {
        repeats *= 2;
    }
    let mut pointer_ns = vec![];
    let mut enum_ns = vec![];
    for sample in 0..21 {
        let (p, e) = if sample % 2 == 0 {
            let p = time(pointer, h, repeats);
            let e = time(en, h, repeats);
            (p, e)
        } else {
            let e = time(en, h, repeats);
            let p = time(pointer, h, repeats);
            (p, e)
        };
        let scale = 1e9 / (repeats * operations) as f64;
        pointer_ns.push(p * scale);
        enum_ns.push(e * scale);
        std::thread::sleep(Duration::from_millis(2));
    }
    let ratios = enum_ns
        .iter()
        .zip(&pointer_ns)
        .map(|(e, p)| e / p)
        .collect::<Vec<_>>();
    let p = percentile(&pointer_ns, 0.5);
    let e = percentile(&enum_ns, 0.5);
    let ratio = percentile(&ratios, 0.5);
    eprintln!(
        "{label}: pointer={p:.2} ns/op, enum={e:.2} ns/op, paired ratio={ratio:.3}x, p10..p90={:.3}..{:.3}",
        percentile(&ratios, 0.1),
        percentile(&ratios, 0.9)
    );
    serde_json::json!({"workload":label,"operations_per_sweep":operations,"sweeps_per_sample":repeats,"samples":21,
        "pointer_median_ns":p,"enum_median_ns":e,"paired_ratio_median":ratio,"paired_ratio_p10":percentile(&ratios,0.1),"paired_ratio_p90":percentile(&ratios,0.9),
        "pointer_samples_ns":pointer_ns,"enum_samples_ns":enum_ns})
}
fn main() {
    let mut results = vec![];
    for (order, h) in [
        ("grouped", Harness::new(false)),
        ("shuffled", Harness::new(true)),
        ("neeko", Harness::single(new::ChampionId::Neeko)),
        ("leblanc", Harness::single(new::ChampionId::Leblanc)),
    ] {
        eprintln!(
            "{order}: {} champions, {} formula calls, max {} abilities",
            h.games_new.len(),
            h.formulas_new.len(),
            h.metadata.iter().map(|m| m.len()).max().unwrap()
        );
        results.push(measure(
            &format!("formulas_{order}"),
            &h,
            pointer_formulas,
            enum_formulas,
            h.formulas_new.len(),
        ));
        results.push(measure(
            &format!("batches_{order}"),
            &h,
            pointer_batches,
            enum_batches,
            h.batches_new.len(),
        ));
        results.push(measure(
            &format!("ability_rows_{order}"),
            &h,
            pointer_rows,
            enum_rows,
            h.batches_new.len(),
        ));
        results.push(measure(
            &format!("calculator_{order}"),
            &h,
            pointer_calculator,
            enum_calculator,
            h.games_new.len(),
        ));
    }
    println!("{}",serde_json::to_string_pretty(&serde_json::json!({"profile":"release: opt-level=3, fat LTO, codegen-units=1; both variants in the same executable","experimental_features":{"engine":cfg!(feature="engine"),"client_data":cfg!(feature="client-data"),"formula_docs":cfg!(feature="formula-docs"),"yew":cfg!(feature="yew")},"baseline_features":"engine only","formula_enum_bytes":size_of::<new::Closure>(),"function_pointer_bytes":size_of::<OldFormula>(),"batch_enum_bytes":size_of::<new::AbilityBatch>(),"results":results})).unwrap());
}
