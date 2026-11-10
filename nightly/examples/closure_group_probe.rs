//! Standalone validation only; the library still uses enum dispatch.
#![feature(const_trait_impl, const_closures)]
#![allow(incomplete_features, unexpected_cfgs)]

use tutorlolv2::{AbilityId, AbilityName, Attrs, Ctx, DamageType, TypeMetadata, model::Modifiers};

pub const trait ClosureGroup {
    fn len(&self) -> usize;
    fn eval_at(&self, index: usize, ctx: &Ctx) -> f32;
}

macro_rules! tuple_groups {
    ($(($len:literal; $($ty:ident:$field:tt),*)),* $(,)?) => {$ (
        const impl<$($ty,)*> ClosureGroup for ($($ty,)*)
        where $($ty: [const] Fn(&Ctx) -> f32,)*
        {
            fn len(&self) -> usize { $len }
            fn eval_at(&self, index: usize, ctx: &Ctx) -> f32 {
                // The unit tuple has no callable fields.
                let _ = ctx;
                match index {
                    $($field => (self.$field)(ctx),)*
                    _ => panic!("closure group index out of bounds"),
                }
            }
        }
    )*};
}

tuple_groups! {
    (0; ),
    (1; F0:0),
    (2; F0:0, F1:1),
    (3; F0:0, F1:1, F2:2),
    (4; F0:0, F1:1, F2:2, F3:3),
    (5; F0:0, F1:1, F2:2, F3:3, F4:4),
    (6; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5),
    (7; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6),
    (8; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7),
    (9; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8),
    (10; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9),
    (11; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10),
    (12; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11),
    (13; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12),
    (14; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13),
    (15; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13, F14:14),
    (16; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13, F14:14, F15:15),
    (17; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13, F14:14, F15:15, F16:16),
    (18; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13, F14:14, F15:15, F16:16, F17:17),
}

macro_rules! const_group {
    ($($f:expr),* $(,)?) => { ($($f,)*) };
}

pub const fn eval<C: [const] ClosureGroup>(
    ctx: &Ctx,
    modifiers: Modifiers,
    metadata: &[TypeMetadata<AbilityId>],
    closures: &C,
    out: &mut [i32],
) {
    assert!(metadata.len() == closures.len(), "metadata length mismatch");
    assert!(out.len() == metadata.len(), "output length mismatch");
    let mut i = 0;
    while i < metadata.len() {
        let meta = metadata[i];
        let modifier = tutorlolv2::helpers::ability_id_mod(meta.kind, meta.damage_type, modifiers);
        out[i] = (modifier * closures.eval_at(i, ctx)) as i32;
        i += 1;
    }
}

// Item/rune evaluation has TWO callables per metadata record, unlike abilities.
const fn eval_pairs<C: [const] ClosureGroup, Meta>(
    ctx: &Ctx,
    modifiers: Modifiers,
    metadata: &[TypeMetadata<Meta>],
    closures: &C,
    out: &mut [i32],
    onhit: &mut tutorlolv2::model::RangeDamage,
) {
    assert!(closures.len() == metadata.len() * 2);
    assert!(out.len() == closures.len());
    let mut i = 0;
    while i < metadata.len() {
        let modifier = modifiers.damages.modifier(metadata[i].damage_type);
        let mut j = 0;
        while j < 2 {
            let index = i * 2 + j;
            let damage = (modifier * closures.eval_at(index, ctx)) as i32;
            out[index] = damage;
            onhit.inc_attr(metadata[i].attributes, damage);
            j += 1;
        }
        i += 1;
    }
}

const CTX: Ctx = Ctx {
    attack_damage: 100.0,
    ability_power: 50.0,
    ..unsafe { core::mem::zeroed() }
};
const META: TypeMetadata<AbilityId> = TypeMetadata {
    kind: AbilityId::Q(AbilityName::Void),
    damage_type: DamageType::Physical,
    attributes: Attrs::Undefined,
};
const fn base(ctx: &Ctx) -> f32 {
    ctx.attack_damage
}
const fn double(ctx: &Ctx) -> f32 {
    ctx.attack_damage * 2.0
}
const fn ap(ctx: &Ctx) -> f32 {
    ctx.ability_power
}

const fn compute<C: [const] ClosureGroup, const N: usize>(group: &C) -> [i32; N] {
    let mut out = [0; N];
    eval(&CTX, Modifiers::default(), &[META; N], group, &mut out);
    out
}

const EMPTY: [i32; 0] = compute(&const_group![]);
const SINGLE: [i32; 1] = compute(&const_group![base]);
const MIXED: [i32; 3] = compute(&const_group![base, double, const |ctx: &Ctx| ctx
    .ability_power
    * 2.0]);
const EIGHTEEN: [i32; 18] = compute(&const_group![
    base, double, ap, base, double, ap, base, double, ap, base, double, ap, base, double, ap, base,
    double, ap
]);

const CAPTURED: [i32; 2] = {
    let scale = 3.0;
    compute(&const_group![base, const move |ctx: &Ctx| ctx
        .ability_power
        * scale])
};

const ITEM_PAIR: ([i32; 2], tutorlolv2::model::RangeDamage) = {
    let mut out = [0; 2];
    let mut onhit = tutorlolv2::model::RangeDamage::default();
    eval_pairs(
        &CTX,
        Modifiers::default(),
        &[tutorlolv2::ItemId::NashorsTooth.metadata()],
        &const_group![
            tutorlolv2::libgen::items::nashors_tooth::nashors_tooth,
            tutorlolv2::libgen::zero
        ],
        &mut out,
        &mut onhit,
    );
    (out, onhit)
};

// Different tuple types work behind one runtime selection when each match arm
// calls the generic evaluator instead of returning its group.
const fn eval_champion(id: tutorlolv2::ChampionId, ctx: &Ctx, out: &mut [i32]) {
    use tutorlolv2::libgen::champions::{leblanc::*, neeko::*};
    match id {
        tutorlolv2::ChampionId::Neeko => eval(
            ctx,
            Modifiers::default(),
            id.metadata(),
            &const_group![neeko_q1, neeko_qmin, neeko_qmax, neeko_w, neeko_e, neeko_r],
            out,
        ),
        tutorlolv2::ChampionId::Leblanc => eval(
            ctx,
            Modifiers::default(),
            id.metadata(),
            &const_group![
                leblanc_qmin,
                leblanc_qmax,
                leblanc_w,
                leblanc_e1,
                leblanc_emin,
                leblanc_emax,
                leblanc_r1,
                leblanc_r2,
                leblanc_r3,
                leblanc_r4,
                leblanc_r5,
                leblanc_r6
            ],
            out,
        ),
        _ => panic!("probe only covers Neeko and Leblanc"),
    }
}

const NEEKO: [i32; 6] = {
    let mut out = [0; 6];
    eval_champion(tutorlolv2::ChampionId::Neeko, &CTX, &mut out);
    out
};
const LEBLANC: [i32; 12] = {
    let mut out = [0; 12];
    eval_champion(tutorlolv2::ChampionId::Leblanc, &CTX, &mut out);
    out
};

#[cfg(invalid_index)]
const INVALID: f32 = const_group![base].eval_at(1, &CTX);

fn main() {
    assert_eq!(EMPTY, []);
    let scale = 3.0;
    assert_eq!(CAPTURED, [100, 150]);
    assert_eq!(
        CAPTURED,
        compute(&const_group![base, move |ctx: &Ctx| ctx.ability_power * scale])
    );
    assert_eq!(SINGLE, [100]);
    assert_eq!(MIXED, [100, 200, 100]);
    assert_eq!(
        EIGHTEEN,
        [
            100, 200, 50, 100, 200, 50, 100, 200, 50, 100, 200, 50, 100, 200, 50, 100, 200, 50
        ]
    );
    assert_eq!(EMPTY, compute(&const_group![]));
    assert_eq!(SINGLE, compute(&const_group![base]));
    assert_eq!(
        MIXED,
        compute(&const_group![base, double, |ctx: &Ctx| ctx.ability_power * 2.0])
    );
    assert_eq!(
        EIGHTEEN,
        compute(&const_group![
            base, double, ap, base, double, ap, base, double, ap, base, double, ap, base, double,
            ap, base, double, ap
        ])
    );
    // Distinct function item types occupy no storage; no fn-pointer coercions.
    assert_eq!(size_of_val(&const_group![base, double, ap]), 0);
    let mut pair = [0; 2];
    let mut onhit = tutorlolv2::model::RangeDamage::default();
    let item = tutorlolv2::ItemId::NashorsTooth;
    tutorlolv2::model::DamageTable::eval(
        &CTX,
        &mut onhit,
        Modifiers::default(),
        &[item.metadata()],
        &item.data().ranged,
        &mut pair,
    );
    assert_eq!(ITEM_PAIR, (pair, onhit));
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    assert!(std::panic::catch_unwind(|| ().eval_at(0, &CTX)).is_err());
    assert!(std::panic::catch_unwind(|| const_group![base].eval_at(1, &CTX)).is_err());
    assert!(std::panic::catch_unwind(|| const_group![base, double].eval_at(2, &CTX)).is_err());
    std::panic::set_hook(hook);
    let mut neeko = [0; 6];
    let mut leblanc = [0; 12];
    eval_champion(
        std::hint::black_box(tutorlolv2::ChampionId::Neeko),
        &CTX,
        &mut neeko,
    );
    eval_champion(
        std::hint::black_box(tutorlolv2::ChampionId::Leblanc),
        &CTX,
        &mut leblanc,
    );
    assert_eq!(NEEKO, neeko);
    assert_eq!(LEBLANC, leblanc);
    for (id, expected) in [
        (tutorlolv2::ChampionId::Neeko, NEEKO.as_slice()),
        (tutorlolv2::ChampionId::Leblanc, LEBLANC.as_slice()),
    ] {
        let mut out = vec![0; expected.len()];
        let mut onhit = tutorlolv2::model::RangeDamage::default();
        tutorlolv2::model::DamageTable::eval_abilities(
            &CTX,
            &mut onhit,
            Modifiers::default(),
            id.metadata(),
            id.data().closures,
            &mut out,
        );
        assert_eq!(expected, out.as_slice());
    }
    println!(
        "Tuple arities 0..18 compile; empty, single, mixed, and 18-element const/runtime evaluations agree; invalid runtime indices panic."
    );
}
