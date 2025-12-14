use super::model::*;
use crate::{ability_id_mod, eval_attacks};
use tutorlolv2_gen::*;

/// Constant version of the [`eval_damage`] function for the enum [`AbilityId`].
pub const fn const_ability_id_eval_damage<const N: usize>(
    ctx: &EvalContext,
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

pub const fn const_item_id_eval_damage<const N: usize>(
    ctx: &EvalContext,
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
            attributes,
            damage_type,
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

pub const fn const_rune_id_eval_damage<const N: usize>(
    ctx: &EvalContext,
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

/// Constructs a new [`Damages`] struct that holds all the damage values against some entity
/// that could be calculated. This function will cause undefined behavior if any
/// metadata of closures vectors do not have the same length
pub const fn get_damages<const A: usize, const I: usize, const R: usize>(
    eval_ctx: &EvalContext,
    attack_type: AttackType,
    champion_id: ChampionId,
    item_ids: [ItemId; I],
    rune_ids: [RuneId; R],
    modifiers: Modifiers,
) -> ConstDamages<A, I, R> {
    let mut onhit = RangeDamage::default();

    let abilities = const_ability_id_eval_damage(&eval_ctx, &mut onhit, champion_id, modifiers);
    let items = const_item_id_eval_damage(&eval_ctx, &mut onhit, item_ids, attack_type, modifiers);
    let runes = const_rune_id_eval_damage(&eval_ctx, rune_ids, attack_type, modifiers);
    let attacks = eval_attacks(&eval_ctx, onhit, modifiers.damages.physical_mod);

    ConstDamages {
        abilities,
        items,
        runes,
        attacks,
    }
}
