//! This module exports functions that can be used to evaluate
//! at compile time the damages of abilities, items, and runes.
//! Since dynamic allocation is illegal in const context, those
//! functions require constant generic parameters that serve as
//! lengths of the returned arrays. For the case of items and
//! runes, it can be inferred by Rust's compiler. However, for
//! abilities, you'll have to get the number of abilities that
//! the champion you're trying to evaluate has, and pass it as
//! a generic parameter to the function. See its documentation
//! for more details

use crate::{
    helpers::ability_id_mod,
    model::{Modifiers, RangeDamage},
};
use tutorlolv2_gen::{
    AttackType, CHAMPION_CACHE, CachedChampion, CachedItem, CachedRune, ChampionId, Ctx,
    ITEM_CACHE, ItemId, RUNE_CACHE, RuneId, TypeMetadata, champions::ability_const_eval,
    items::item_const_eval, runes::rune_const_eval,
};

/// Constant evaluation of abilities, similar to function [`crate::helpers::ability_id_eval_damage`]
/// Let's say you're trying to evaluate the damage of Neeko, which means you'll provide
/// a [`ChampionId::Neeko`] as argument, in order to know what the const generic parameter
/// `N` should be, you can do the following
///
/// ```rs
/// const N: usize = tutorlolv2::champions::NEEKO.closures.len();
/// const CHAMPION_ID: ChampionId = ChampionId::Neeko;
/// const N: usize = CHAMPION_CACHE[CHAMPION_ID as usize].closures.len();
/// ```
///
/// or (not recommended) you can use an arbitrary `N` value and keep decreasing it
/// until the compiler doesn't emit any `Index out of bounds` error
///
/// Example of usage, assuming you haven't got the necessary structs such as
/// [`Ctx`] [`crate::model::SelfState`], and [`crate::model::EnemyState`]
/// from previous constant assignments
/// ```rs
/// const NEEKO_ABILITIES: usize = tutorlolv2::NEEKO.closures.len();
/// const NEEKO_DAMAGES: [i32; NEEKO_ABILITIES] = const_ability_id_eval_damage(
///     &tutorlolv2::helpers::get_eval_ctx(
///         &SelfState {
///             ability_levels: AbilityLevels {
///                 q: 5,
///                 w: 5,
///                 e: 5,
///                 r: 3,
///             },
///             current_stats: Stats::default(),
///             bonus_stats: BasicStats::default(),
///             base_stats: BasicStats::default(),
///             level: 18,
///             adaptative_type: AdaptativeType::Magic,
///         },
///         &tutorlolv2::helpers::get_enemy_state(
///             EnemyState {
///                 base_stats: SimpleStats::default(),
///                 items: &[],
///                 stacks: 0,
///                 champion_id: ChampionId::Aatrox,
///                 earth_dragons: 0,
///                 level: 18,
///                 item_exceptions: &[],
///             },
///             ResistShred {
///                 armor_penetration_flat: 0.0,
///                 armor_penetration_percent: 0.0,
///                 magic_penetration_flat: 0.0,
///                 magic_penetration_percent: 0.0,
///             },
///             false,
///         ),
///     ),
///     &mut RangeDamage::default(),
///     ChampionId::Neeko,
///     Modifiers::default(),
/// );
/// ```
///
/// When hovering over the constant `NEEKO_DAMAGES`, you should be able to
/// see the resolved numbers for the damage of each of her abilities
pub const fn const_ability_id_eval_damage<const N: usize>(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    champion_id: ChampionId,
    modifiers: Modifiers,
) -> [i32; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < N {
        let CachedChampion { metadata, .. } = CHAMPION_CACHE[champion_id as usize];
        let TypeMetadata {
            kind,
            damage_type,
            attributes,
        } = metadata[i];
        let modifier = ability_id_mod(kind, damage_type, modifiers);
        let damage = (modifier * ability_const_eval(ctx, champion_id, kind)) as i32;
        onhit.inc_attr(attributes, damage);
        result[i] = damage;
        i += 1;
    }
    result
}

/// Constant evaluation of items, similar to function [`crate::helpers::item_id_eval_damage`]
pub const fn const_item_id_eval_damage<const N: usize>(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    item_ids: [ItemId; N],
    attack_type: AttackType,
    modifiers: Modifiers,
) -> [i32; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < N {
        let item_id = item_ids[i];
        let CachedItem {
            metadata:
                TypeMetadata {
                    damage_type,
                    attributes,
                    ..
                },
            ..
        } = ITEM_CACHE[item_id as usize];
        let modifier = modifiers.damages.modifier(*damage_type);
        let damages = item_const_eval(ctx, item_id, attack_type);
        let mut j = 0;
        while j < 2 {
            let damage = (modifier * damages[j]) as i32;
            onhit.inc_attr(*attributes, damage);
            result[i + j] = damage;
            j += 1;
        }
        i += 2;
    }
    result
}

/// Constant evaluation of runes, similar to function [`crate::helpers::rune_id_eval_damage`].
pub const fn const_rune_id_eval_damage<const N: usize>(
    ctx: &Ctx,
    rune_ids: [RuneId; N],
    attack_type: AttackType,
    modifiers: Modifiers,
) -> [i32; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < N {
        let rune_id = rune_ids[i];
        let CachedRune { damage_type, .. } = RUNE_CACHE[rune_id as usize];
        let modifier = modifiers.damages.modifier(*damage_type);
        result[i] = (modifier * rune_const_eval(ctx, rune_id, attack_type)) as i32;
        i += 1;
    }
    result
}
