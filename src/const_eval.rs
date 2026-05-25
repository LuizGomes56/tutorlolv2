use crate::{
    helpers::ability_id_mod,
    model::{ConstDamageKind, Modifiers},
};
use tutorlolv2_gen::{
    AttackType, ChampionId, Closure, Ctx, ITEM_CACHE, ItemId, ItemsBitSet, RUNE_CACHE, RuneId,
    RunesBitSet, TypeMetadata,
};

pub const fn get_items_data_const<const N: usize, const L: usize>(
    items: &ItemsBitSet,
    attack_type: AttackType,
) -> ConstDamageKind<ItemId, N, L> {
    assert!(L == N << 1);
    unsafe {
        let mut metadata: [TypeMetadata<ItemId>; N] = core::mem::zeroed();
        let mut closures: [Closure; L] = core::mem::zeroed();

        let mut i = 0;
        let mut j = 0;

        let mut iter = items.iter_const();

        while let Some(index) = iter.next_const() {
            let item = ITEM_CACHE[index as usize];
            let slice = match attack_type {
                AttackType::Ranged => item.ranged,
                AttackType::Melee => item.melee,
            };

            metadata[i] = item.metadata;
            closures[j] = slice[0];
            closures[j + 1] = slice[1];

            i += 1;
            j += 2;
        }

        assert!(i == N);
        assert!(j == L);

        ConstDamageKind { metadata, closures }
    }
}

pub const fn get_runes_data_const<const N: usize, const L: usize>(
    runes: &RunesBitSet,
    attack_type: AttackType,
) -> ConstDamageKind<RuneId, N, L> {
    assert!(L == N << 1);
    unsafe {
        let mut metadata: [TypeMetadata<RuneId>; N] = core::mem::zeroed();
        let mut closures: [Closure; L] = core::mem::zeroed();

        let mut i = 0;
        let mut j = 0;

        let mut iter = runes.iter_const();

        while let Some(index) = iter.next_const() {
            let rune = RUNE_CACHE[index as usize];
            let slice = match attack_type {
                AttackType::Ranged => rune.ranged,
                AttackType::Melee => rune.melee,
            };

            metadata[i] = rune.metadata;
            closures[j] = slice[0];
            closures[j + 1] = slice[1];

            i += 1;
            j += 2;
        }

        assert!(i == N);
        assert!(j == L);

        ConstDamageKind { metadata, closures }
    }
}

pub const fn const_ability_id_eval_damage<const N: usize>(
    ctx: &Ctx,
    champion_id: ChampionId,
    modifiers: Modifiers,
) -> [i32; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < N {
        let metadata = champion_id.metadata();

        let TypeMetadata {
            kind, damage_type, ..
        } = metadata[i];

        let modifier = ability_id_mod(kind, damage_type, modifiers);
        let damage = (modifier * champion_id.eval(ctx, kind)) as i32;
        result[i] = damage;
        i += 1;
    }
    result
}

pub const fn eval_item_damage_const<const N: usize, const L: usize>(
    ctx: &Ctx,
    item_ids: [ItemId; N],
    attack_type: AttackType,
    modifiers: Modifiers,
) -> [i32; L] {
    assert!(L == N << 1, "Generic argument #2 must be #1 * 2");
    let mut result = [0i32; L];
    let mut i = 0usize;
    let mut j = 0usize;

    while i < N {
        let item_id = item_ids[i];
        let damage_type = item_id.damage_type();
        let modifier = modifiers.damages.modifier(damage_type);
        let damages = item_id.eval(ctx, attack_type);
        let mut k = 0usize;

        while k < 2 {
            let damage = (modifier * damages[k]) as i32;
            result[j + k] = damage;
            k += 1;
        }

        i += 1;
        j += 2;
    }

    result
}

pub const fn eval_rune_damage_const<const N: usize, const L: usize>(
    ctx: &Ctx,
    rune_ids: [RuneId; N],
    attack_type: AttackType,
    modifiers: Modifiers,
) -> [i32; L] {
    assert!(L == N << 1, "Generic argument #2 must be #1 * 2");
    let mut result = [0i32; L];
    let mut i = 0usize;
    let mut j = 0usize;

    while i < N {
        let rune_id = rune_ids[i];
        let damage_type = rune_id.damage_type();
        let modifier = modifiers.damages.modifier(damage_type);
        let damages = rune_id.eval(ctx, attack_type);
        let mut k = 0usize;

        while k < 2 {
            let damage = (modifier * damages[k]) as i32;
            result[j + k] = damage;
            k += 1;
        }

        i += 1;
        j += 2;
    }

    result
}
