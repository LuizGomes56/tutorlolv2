//! Benchmark snapshot: same storage/preparation for every dispatch mode.
use crate::groups::{self, ClosureGroup};
use new::{
    calculator::{EnemyMap, InferStats},
    helpers::*,
    model::*,
    *,
};
use std::num::NonZeroU8;

pub trait Dispatch {
    fn abilities(
        &self,
        id: ChampionId,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    );
    fn item(
        &self,
        id: ItemId,
        a: AttackType,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    );
    fn rune(
        &self,
        id: RuneId,
        a: AttackType,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    );
}

pub struct Pointers<const B: bool> {
    pub id: ChampionId,
}
pub struct Enums<const B: bool> {
    pub id: ChampionId,
}
pub struct Tuples<C> {
    pub group: C,
}

impl<const B: bool> Dispatch for Pointers<B> {
    #[inline]
    fn abilities(
        &self,
        id: ChampionId,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    ) {
        if B {
            (groups::pointer_batch(self.id))(ctx, m, onhit, out)
        } else {
            groups::pointer_abilities(
                ctx,
                onhit,
                m,
                id.metadata(),
                groups::pointers_champions_closures(id),
                out,
            )
        }
    }
    #[inline]
    fn item(
        &self,
        id: ItemId,
        a: AttackType,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    ) {
        let fs = match a {
            AttackType::Melee => groups::pointers_items_melee(id),
            AttackType::Ranged => groups::pointers_items_ranged(id),
        };
        groups::pointer_pairs(ctx, onhit, m, &[id.metadata()], fs, out);
    }
    #[inline]
    fn rune(
        &self,
        id: RuneId,
        a: AttackType,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    ) {
        let fs = match a {
            AttackType::Melee => groups::pointers_runes_melee(id),
            AttackType::Ranged => groups::pointers_runes_ranged(id),
        };
        groups::pointer_pairs(ctx, onhit, m, &[id.metadata()], fs, out);
    }
}
impl<const B: bool> Dispatch for Enums<B> {
    #[inline]
    fn abilities(
        &self,
        id: ChampionId,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    ) {
        if B {
            (self.id.data().ability_batch)(ctx, m, onhit, out)
        } else {
            new::model::DamageTable::eval_abilities(
                ctx,
                onhit,
                m,
                id.metadata(),
                id.data().closures,
                out,
            )
        }
    }
    #[inline]
    fn item(
        &self,
        id: ItemId,
        a: AttackType,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    ) {
        let data = id.data();
        let fs = match a {
            AttackType::Melee => data.melee,
            AttackType::Ranged => data.ranged,
        };
        new::model::DamageTable::eval(ctx, onhit, m, &[id.metadata()], &fs, out);
    }
    #[inline]
    fn rune(
        &self,
        id: RuneId,
        a: AttackType,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    ) {
        let data = id.data();
        let fs = match a {
            AttackType::Melee => data.melee,
            AttackType::Ranged => data.ranged,
        };
        new::model::DamageTable::eval(ctx, onhit, m, &[id.metadata()], &fs, out);
    }
}
impl<C: ClosureGroup> Dispatch for Tuples<C> {
    #[inline]
    fn abilities(
        &self,
        id: ChampionId,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    ) {
        groups::abilities(ctx, onhit, m, id.metadata(), &self.group, out);
    }
    #[inline]
    fn item(
        &self,
        id: ItemId,
        a: AttackType,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    ) {
        match a {
            AttackType::Melee => groups::tuple_eval_items_melee(id, ctx, out, onhit, m),
            AttackType::Ranged => groups::tuple_eval_items_ranged(id, ctx, out, onhit, m),
        }
    }
    #[inline]
    fn rune(
        &self,
        id: RuneId,
        a: AttackType,
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        m: Modifiers,
        out: &mut [i32],
    ) {
        match a {
            AttackType::Melee => groups::tuple_eval_runes_melee(id, ctx, out, onhit, m),
            AttackType::Ranged => groups::tuple_eval_runes_ranged(id, ctx, out, onhit, m),
        }
    }
}

pub struct Output {
    pub damages: Table,
    pub current_player: OutputCurrentPlayer,
    pub enemies: Box<[OutputEnemy]>,
    pub tower_damages: [i32; L_TWRD],
    pub items_meta: Box<[TypeMetadata<ItemId>]>,
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,
}
pub struct Table {
    pub layout: DamageLayout,
    pub values: Box<[i32]>,
    pub attackers: Box<[CtxAttacker]>,
    pub targets: Box<[CtxTarget]>,
}
impl Table {
    fn new(layout: DamageLayout, rows: usize) -> Self {
        Self {
            layout,
            values: unsafe { Box::new_zeroed_slice(rows * layout.stride()).assume_init() },
            attackers: unsafe { Box::new_zeroed_slice(1).assume_init() },
            targets: unsafe { Box::new_zeroed_slice(rows).assume_init() },
        }
    }
    pub fn row(&self, r: usize) -> Damages<'_> {
        let stride = self.layout.stride();
        let row = &self.values[r * stride..(r + 1) * stride];
        let (a, rest) = row.split_at(4);
        let (abilities, rest) = rest.split_at(self.layout.abilities as usize);
        let (items, runes) = rest.split_at(self.layout.items as usize);
        Damages {
            attacks: Attacks {
                basic_attack: a[0],
                critical_strike: a[1],
                onhit_damage: RangeDamage {
                    minimum_damage: a[2],
                    maximum_damage: a[3],
                },
            },
            abilities,
            items,
            runes,
            ctx: Ctx::new(&self.attackers[0], &self.targets[r]),
        }
    }
    fn fill<M: Dispatch>(
        &mut self,
        mode: &M,
        id: ChampionId,
        a: AttackType,
        r: usize,
        ctx: Ctx,
        data: &DamageEvalData,
        m: Modifiers,
    ) {
        if r == 0 {
            self.attackers[0] = ctx.attacker();
        }
        self.targets[r] = ctx.target();
        let mut view = DamageTableMut::new(
            self.layout,
            &mut self.values,
            &mut self.attackers,
            &mut self.targets,
        );
        let row = view.row_mut(r);
        let mut onhit = RangeDamage::default();
        mode.abilities(id, &ctx, &mut onhit, m, row.abilities);
        for (i, meta) in data.items.metadata.iter().enumerate() {
            mode.item(
                meta.kind,
                a,
                &ctx,
                &mut onhit,
                m,
                &mut row.items[i * 2..i * 2 + 2],
            );
        }
        for (i, meta) in data.runes.metadata.iter().enumerate() {
            mode.rune(
                meta.kind,
                a,
                &ctx,
                &mut onhit,
                m,
                &mut row.runes[i * 2..i * 2 + 2],
            );
        }
        let attacks = Attacks::new(&ctx, onhit, m.damages.physical_mod);
        *row.attacks = [
            attacks.basic_attack,
            attacks.critical_strike,
            attacks.onhit_damage.minimum_damage,
            attacks.onhit_damage.maximum_damage,
        ];
    }
}

struct PreparedCalculator {
    self_state: SelfState,
    shred: ResistShred,
    modifiers: Modifiers,
    tower_damages: [i32; new::L_TWRD],
    items: new::bitset::BitSet,
    runes: new::bitset::BitSet,
    champion_id: ChampionId,
    attack_type: AttackType,
}

impl PreparedCalculator {
    const fn current_player(&self) -> OutputCurrentPlayer {
        OutputCurrentPlayer {
            base_stats: self.self_state.base_stats,
            bonus_stats: self
                .self_state
                .current_stats
                .bonus_stats(self.self_state.base_stats),
            current_stats: self.self_state.current_stats,
            champion_id: self.champion_id,
            adaptive_type: self.self_state.adaptive_type,
            level: self.self_state.level,
        }
    }
}

const fn prepare(game: InputGame<'_>) -> PreparedCalculator {
    let InputGame {
        active_player:
            InputActivePlayer {
                abilities: ability_levels,
                runes: current_player_raw_runes,
                rune_exceptions,
                current_health,
                data:
                    InputMinData {
                        stats: champion_raw_stats,
                        level,
                        items: current_player_raw_items,
                        champion_id: current_player_champion_id,
                        is_mega_gnar,
                        stacks,
                        item_exceptions,
                    },
            },
        dragons,
        enemy_players: _,
        game_map,
    } = game;

    let mut modifiers = const { Modifiers::default() };
    let current_player_cache = current_player_champion_id.data();
    let current_player_base_stats =
        current_player_champion_id.map_base_stats(level, is_mega_gnar, game_map);

    let champion_stats = match champion_raw_stats {
        Some(stats) => {
            modifiers.apply_loadout(current_player_raw_items, current_player_raw_runes, &stats);
            stats
        }
        None => InferStats {
            item_exceptions,
            rune_exceptions,
            items: current_player_raw_items,
            runes: current_player_raw_runes,
            modifiers: &mut modifiers,
            dragons,
            ability_levels,
            stacks,
            level,
            champion_id: current_player_champion_id,
            is_mega_gnar,
            current_health,
            game_map,
        }
        .infer(),
    };

    let current_player_bonus_stats = champion_stats.bonus_stats(current_player_base_stats);

    let adaptive_type = match AdaptiveType::try_infer(
        current_player_bonus_stats.attack_damage,
        champion_stats.ability_power,
    ) {
        Some(x) => x,
        None => current_player_cache.adaptive_type,
    };

    let shred = ResistShred::new(&champion_stats);
    let tower_damages = get_tower_damages(
        adaptive_type,
        current_player_base_stats.attack_damage,
        current_player_bonus_stats.attack_damage,
        champion_stats.ability_power,
        shred,
    );

    let current_player_runes = get_damaging_runes(current_player_raw_runes);
    let current_player_items = get_damaging_items(current_player_raw_items);

    let self_state = SelfState {
        stacks: stacks as _,
        current_stats: champion_stats,
        base_stats: current_player_base_stats,
        adaptive_type,
        ability_levels,
        level,
    };

    let attack_type = current_player_champion_id.effective_attack_type(is_mega_gnar);

    PreparedCalculator {
        self_state,
        shred,
        modifiers,
        tower_damages,
        items: current_player_items,
        runes: current_player_runes,
        champion_id: current_player_champion_id,
        attack_type,
    }
}

// One identical allocator/inference pipeline, monomorphized for each strategy.
// In the tuple case the group is selected once before entering this function.
#[inline(never)]
pub fn calculator<M: Dispatch>(game: InputGame<'_>, mode: &M) -> Output {
    let p = prepare(game);
    let champion = p.champion_id.data();
    let data = DamageEvalData {
        abilities: StaticDamageKind {
            metadata: champion.metadata,
            closures: champion.closures,
        },
        items: DamageKind::items(&p.items, p.attack_type),
        runes: DamageKind::runes(&p.runes, p.attack_type),
    };
    let mut damages = Table::new(
        new::calculator::calculator_layout(game),
        L_MSTR + game.enemy_players.len(),
    );
    let mut enemies = vec![
        OutputEnemy {
            base_stats: SimpleStats::default(),
            bonus_stats: SimpleStats::default(),
            current_stats: EnemyStats {
                armor: 0.0,
                magic_resist: 0.0,
                max_health: 0.0,
                current_health: 0.0
            },
            real_armor: 0,
            real_magic_resist: 0,
            champion_id: ChampionId::default(),
            level: NonZeroU8::MIN
        };
        game.enemy_players.len()
    ]
    .into_boxed_slice();
    for (i, &(armor, magic_resist)) in RiotFormulas::MONSTER_RESISTS.iter().enumerate() {
        let full = EnemyState {
            base_stats: SimpleStats {
                armor,
                max_health: 1000.0,
                magic_resist,
            },
            ..Default::default()
        }
        .full_state(p.shred, true);
        damages.fill(
            mode,
            p.champion_id,
            p.attack_type,
            i,
            p.self_state.ctx(&full),
            &data,
            p.modifiers.against(&full, p.self_state.adaptive_type),
        );
    }
    for (e, enemy) in game.enemy_players.iter().enumerate() {
        let (ctx, m, out) = prepare_enemy(
            enemy,
            &p.self_state,
            p.shred,
            p.modifiers,
            game.dragons.enemy_earth_dragons,
            EnemyMap {
                champion_id: p.champion_id,
                game_map: game.game_map,
            },
        );
        damages.fill(
            mode,
            p.champion_id,
            p.attack_type,
            L_MSTR + e,
            ctx,
            &data,
            m,
        );
        enemies[e] = out;
    }
    Output {
        current_player: p.current_player(),
        tower_damages: p.tower_damages,
        damages,
        enemies,
        items_meta: data.items.metadata,
        runes_meta: data.runes.metadata,
    }
}

const fn prepare_enemy(
    player: &InputMinData<'_, EnemyStats>,
    self_state: &SelfState,
    shred: ResistShred,
    modifiers: Modifiers,
    enemy_earth_dragons: u16,
    enemy_map: EnemyMap,
) -> (Ctx, Modifiers, OutputEnemy) {
    let EnemyMap {
        champion_id,
        game_map,
    } = enemy_map;
    let InputMinData {
        items: e_items,
        stacks: e_stacks,
        stats: e_stats,
        level: e_level,
        champion_id: e_champion_id,
        is_mega_gnar: e_is_mega_gnar,
        item_exceptions: e_item_exceptions,
    } = *player;

    let e_base_stats = e_champion_id.base_stats(e_level, e_is_mega_gnar).simple();
    let full_state = EnemyState {
        current_stats: e_stats,
        base_stats: e_base_stats,
        items: e_items,
        stacks: e_stacks,
        champion_id: e_champion_id,
        level: e_level,
        item_exceptions: e_item_exceptions,
        earth_dragons: enemy_earth_dragons,
        game_map,
    }
    .full_state(shred, false);

    let ctx = full_state.ctx(self_state);
    let map_mod = champion_id.map_damage_modifier(&e_champion_id, game_map);

    let modifiers = Modifiers {
        damages: DamageModifiers {
            adaptive_type: self_state.adaptive_type,
            physical_mod: modifiers.damages.physical_mod
                * full_state.armor_values.modifier
                * full_state.modifiers.physical_mod,
            magic_mod: modifiers.damages.magic_mod
                * full_state.magic_values.modifier
                * full_state.modifiers.magic_mod,
            true_mod: modifiers.damages.true_mod * full_state.modifiers.true_mod,
            global_mod: modifiers.damages.global_mod * full_state.modifiers.global_mod * map_mod,
        },
        ..modifiers
    };

    (
        ctx,
        modifiers,
        OutputEnemy {
            current_stats: full_state.current_stats,
            bonus_stats: full_state.bonus_stats,
            base_stats: e_base_stats,
            real_magic_resist: full_state.magic_values.real as _,
            real_armor: full_state.armor_values.real as _,
            champion_id: e_champion_id,
            level: e_level,
        },
    )
}
