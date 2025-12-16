use crate::{ability_id_mod, model::*};
use tutorlolv2_gen::*;

/// Constant evaluation of abilities, similar to function [`crate::ability_id_eval_damage`]
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

/// Constant evaluation of items, similar to function [`crate::item_id_eval_damage`]
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

/// Constant evaluation of runes, similar to function [`crate::rune_id_eval_damage`]
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
