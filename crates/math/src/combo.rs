//! Sequential evaluation of a combo against one target. Each step sees the
//! target's health left by the previous ones, so effects that read it (Blade of
//! the Ruined King, Shadowflame, Cut Down, Coup de Grace) are right at every step.
//! Steps can also change the fight: Black Cleaver's armor reduction and the
//! stacks of some champions grow as the combo goes on, see [`FightState`]
//!
//! Before evaluation, a combo is expanded with [`ComboSource::expand`]: procs
//! listed without a basic attack get one inserted, see its documentation
use {
    crate::{
        ChampionId, ItemId, RuneId,
        libgen::{Closure, items},
        model::*,
    },
    alloc::{boxed::Box, vec::Vec},
    types::{
        AbilityId, AbilityName, AttackRole, AttackType, Attrs, Ctx, DamageType, Trigger,
        TypeMetadata,
    },
};

/// Longest combo a request may send, before expansion
pub const MAX_COMBO_STEPS: usize = 64;

/// Largest damage table a combo may produce, in bytes
pub const MAX_TABLE_BYTES: usize = 1 << 20;

/// Guinsoo's Rageblade's Seething Strike applies on-hit effects twice on every
/// this many attacks
pub const SEETHING_STRIKE_ATTACKS: u32 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboError {
    /// The combo has more than [`MAX_COMBO_STEPS`] steps
    TooLong(usize),
    /// The champion has no such ability
    UnknownAbility(AbilityId),
    /// The step names a damaging item or rune the player doesn't own
    NotOwned(ComboStep),
    /// Every-hit effects apply on every attack, so they are never listed
    EveryHitListed(ComboStep),
    /// The damage table would exceed [`MAX_TABLE_BYTES`]
    TooLarge(usize),
}

impl core::fmt::Display for ComboError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooLong(len) => write!(f, "combo of {len} steps, the limit is {MAX_COMBO_STEPS}"),
            Self::UnknownAbility(id) => write!(f, "the champion has no ability {id:?}"),
            Self::NotOwned(step) => write!(f, "{step:?} is not owned"),
            Self::EveryHitListed(step) => {
                write!(f, "{step:?} applies on every attack and can't be listed")
            }
            Self::TooLarge(bytes) => {
                write!(
                    f,
                    "the combo needs {bytes} bytes, the limit is {MAX_TABLE_BYTES}"
                )
            }
        }
    }
}

impl core::error::Error for ComboError {}

/// How a step relates to basic attacks, see [`ComboSource::expand`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StepKind {
    /// A basic attack, or an ability that counts as one
    Attack,
    /// Happens on a basic attack the combo may not list
    Proc,
    Standalone,
}

/// Everything about the attacker that the evaluation of a combo reads
pub struct ComboSource<'a> {
    pub champion_id: ChampionId,
    pub data: &'a DamageEvalData,
    pub owned_items: &'a [ItemId],
    pub owned_runes: &'a [RuneId],
    pub attack_type: AttackType,
    pub level: core::num::NonZeroU8,
}

/// The target of one row, as the table evaluated it
#[derive(Clone, Copy)]
pub struct ComboTarget<'a> {
    pub ctx: Ctx,
    pub state: EnemyFullState,
    /// Final modifiers of the row, health-gated ones included
    pub modifiers: Modifiers,
    pub shred: ResistShred,
    /// Items that count for Shadowflame, which differ for its own candidate row
    pub shadowflame_items: &'a [ItemId],
    /// The simulated purchase of the row, if it deals damage
    pub extra_item: Option<(TypeMetadata<ItemId>, &'a [Closure; 2])>,
}

/// State of the fight that steps change, see [`ChampionId::after_step`]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FightState {
    /// The champion's `ctx.stacks`, which their formulas read
    pub stacks: f32,
    /// A counter a hook may keep beside `stacks`, as Kalista's spears
    pub counter: u8,
}

impl ChampionId {
    /// Changes the fight after `step` dealt its damage. Hand-written for the
    /// champions whose `ctx.stacks` count something in the current fight, see
    /// D17 of the combo spec. `damage` is what the step dealt, and `onhit` is
    /// whether it applied on-hit effects
    pub const fn after_step(
        self,
        step: ComboStep,
        damage: f32,
        onhit: bool,
        fight: &mut FightState,
    ) {
        use ComboStep::*;

        match (self, step) {
            // Each Rampage cast raises the next ones, up to 2 stacks
            (ChampionId::Hecarim, Ability(AbilityId::Q(_))) => {
                fight.stacks = (fight.stacks + 1.0).min(2.0)
            }
            // Rend's `stacks` are the spears after the first. Attacks and Pierce
            // add one, and Rend's spears-after-the-first damage consumes them
            (ChampionId::Kalista, Attack | Crit | Ability(AbilityId::Q(_))) => {
                fight.counter = fight.counter.saturating_add(1);
                fight.stacks = fight.counter.saturating_sub(1) as f32;
            }
            (ChampionId::Kalista, Ability(AbilityId::E(AbilityName::_1))) => {
                fight.counter = 0;
                fight.stacks = 0.0;
            }
            // Plasma: one per hit, up to 4. The rupture consumes them
            (ChampionId::Kaisa, Ability(AbilityId::P(AbilityName::_1))) => fight.stacks = 0.0,
            (ChampionId::Kaisa, _) if onhit => fight.stacks = (fight.stacks + 1.0).min(4.0),
            // Zed's Death Mark and Yone's Soul Unbound store the damage dealt until
            // they pop. The whole combo before the popping ability is taken as
            // stored, so it pops with all of it, and nothing after it is stored
            (ChampionId::Zed, Ability(AbilityId::R(_)))
            | (ChampionId::Yone, Ability(AbilityId::E(_))) => fight.counter = 1,
            (ChampionId::Zed | ChampionId::Yone, _) if fight.counter == 0 => fight.stacks += damage,
            _ => {}
        }
    }
}

impl ComboSource<'_> {
    /// Attributes of an ability step, if the champion has the ability
    fn ability_index(&self, ability_id: AbilityId) -> Result<usize, ComboError> {
        self.data
            .abilities
            .metadata
            .iter()
            .position(|metadata| metadata.kind == ability_id)
            .ok_or(ComboError::UnknownAbility(ability_id))
    }

    fn kind(&self, step: ComboStep) -> Result<StepKind, ComboError> {
        let proc = |attributes: Attrs| match attributes.trigger() {
            Trigger::EveryHit => Err(ComboError::EveryHitListed(step)),
            _ if attributes.implies_attack() => Ok(StepKind::Proc),
            _ => Ok(StepKind::Standalone),
        };

        match step {
            ComboStep::Attack | ComboStep::Crit => Ok(StepKind::Attack),
            ComboStep::OnhitMin | ComboStep::OnhitMax => Ok(StepKind::Proc),
            ComboStep::Ignite => Ok(StepKind::Standalone),
            ComboStep::Ability(ability_id) => {
                let index = self.ability_index(ability_id)?;

                Ok(
                    match self.data.abilities.metadata[index].attributes.role() {
                        AttackRole::IsAttack => StepKind::Attack,
                        _ => StepKind::Standalone,
                    },
                )
            }
            ComboStep::Item(item_id) => {
                match self.owned_items.contains(&item_id) && item_id.deals_damage() {
                    true => proc(item_id.data().metadata.attributes),
                    false => Err(ComboError::NotOwned(step)),
                }
            }
            ComboStep::Rune(rune_id) => {
                match self.owned_runes.contains(&rune_id) && rune_id.deals_damage() {
                    true => proc(rune_id.data().metadata.attributes),
                    false => Err(ComboError::NotOwned(step)),
                }
            }
        }
    }

    /// Validates `combo` and inserts the basic attacks its procs imply (D14). It
    /// is split in *attack groups*, runs of attacks and procs. A group has as
    /// many attacks as the most of what it lists: its attacks, or the repeats of
    /// one proc. Missing attacks are inserted at the start of the group, so every
    /// proc follows the attack it happens on:
    ///
    /// | Combo | Expanded |
    /// |---|---|
    /// | `[Lich Bane, Kraken Slayer]` | `[Attack, Lich Bane, Kraken Slayer]` |
    /// | `[OnhitMin, OnhitMin]` | `[Attack, Attack, OnhitMin, OnhitMin]` |
    /// | `[Q, Lich Bane, W]` | `[Q, Attack, Lich Bane, W]` |
    pub fn expand(&self, combo: &[ComboStep]) -> Result<Box<[ComboStep]>, ComboError> {
        if combo.len() > MAX_COMBO_STEPS {
            return Err(ComboError::TooLong(combo.len()));
        }

        let kinds = combo
            .iter()
            .map(|&step| self.kind(step))
            .collect::<Result<Vec<_>, _>>()?;

        let mut expanded = Vec::with_capacity(combo.len() * 2);
        let mut i = 0;

        while i < combo.len() {
            if kinds[i] == StepKind::Standalone {
                expanded.push(combo[i]);
                i += 1;
                continue;
            }

            let end = (i..combo.len())
                .find(|&j| kinds[j] == StepKind::Standalone)
                .unwrap_or(combo.len());

            let group = &combo[i..end];
            let attacks = kinds[i..end]
                .iter()
                .filter(|&&kind| kind == StepKind::Attack)
                .count();

            let repeats = (i..end)
                .filter(|&j| kinds[j] == StepKind::Proc)
                .map(|j| group.iter().filter(|&&step| step == combo[j]).count())
                .max()
                .unwrap_or(0);

            expanded.extend((attacks..repeats).map(|_| ComboStep::Attack));
            expanded.extend_from_slice(group);
            i = end;
        }

        Ok(expanded.into_boxed_slice())
    }

    /// Damage of an item or rune entry: the larger of its two closures, the
    /// strongest proc
    fn entry_damage(
        ctx: &Ctx,
        modifiers: &Modifiers,
        damage_type: DamageType,
        closures: &[Closure],
    ) -> f32 {
        let modifier = modifiers.damages.modifier(damage_type);

        modifier * closures[0](ctx).max(closures[1](ctx))
    }

    /// Damage of every owned effect whose attributes pass `filter`
    fn sum_entries(
        &self,
        ctx: &Ctx,
        modifiers: &Modifiers,
        extra_item: Option<(TypeMetadata<ItemId>, &[Closure; 2])>,
        filter: impl Fn(Attrs) -> bool,
    ) -> f32 {
        let DamageEvalData {
            items,
            runes,
            rune_stacks,
            ..
        } = self.data;

        let mut sum = 0.0;

        for (metadata, closures) in items.metadata.iter().zip(items.closures.as_chunks::<2>().0) {
            if filter(metadata.attributes) {
                sum += Self::entry_damage(ctx, modifiers, metadata.damage_type, closures);
            }
        }

        if let Some((metadata, closures)) = extra_item
            && filter(metadata.attributes)
        {
            sum += Self::entry_damage(ctx, modifiers, metadata.damage_type, closures);
        }

        for (metadata, closures) in runes.metadata.iter().zip(runes.closures.as_chunks::<2>().0) {
            if filter(metadata.attributes) {
                let ctx = match rune_stacks.get(metadata.kind) {
                    Some(stacks) => &Ctx { stacks, ..*ctx },
                    None => ctx,
                };

                sum += Self::entry_damage(ctx, modifiers, metadata.damage_type, closures);
            }
        }

        sum
    }

    /// Closures of an owned item for this champion's attack type
    fn item_closures(&self, item_id: ItemId) -> &'static [Closure; 2] {
        let item = item_id.data();

        match self.attack_type {
            AttackType::Melee => &item.melee,
            AttackType::Ranged => &item.ranged,
        }
    }

    /// Damage of `step` alone, whether it deals physical damage, whether it
    /// applies on-hit effects and whether it is a basic attack
    fn step_damage(
        &self,
        step: ComboStep,
        ctx: &Ctx,
        modifiers: &Modifiers,
    ) -> (f32, bool, bool, bool) {
        let attack = || (self.data.basic_attack)(ctx);

        match step {
            ComboStep::Attack => (attack(), true, true, true),
            ComboStep::Crit => (attack() * ctx.crit_damage() / 100.0, true, true, true),
            // The implied attack deals the every-hit part
            ComboStep::OnhitMin => (0.0, false, false, false),
            ComboStep::OnhitMax => {
                let procs = self.sum_entries(ctx, modifiers, None, |attributes| {
                    attributes.onhit_max() && !attributes.onhit_min()
                });

                (procs, false, false, false)
            }
            ComboStep::Ignite => (
                modifiers.damages.global_mod
                    * modifiers.damages.true_mod
                    * crate::ignite(self.level) as f32,
                false,
                false,
                false,
            ),
            ComboStep::Ability(ability_id) => {
                let index = self.ability_index(ability_id).unwrap_or_default();
                let TypeMetadata {
                    kind,
                    damage_type,
                    attributes,
                } = self.data.abilities.metadata[index];

                let damage = modifiers.ability_id_mod(kind, damage_type)
                    * self.data.abilities.closures[index](ctx);

                let role = attributes.role();

                (
                    damage,
                    damage_type == DamageType::Physical,
                    role != AttackRole::Standalone,
                    role == AttackRole::IsAttack,
                )
            }
            ComboStep::Item(item_id) => {
                let damage_type = item_id.data().metadata.damage_type;
                let damage =
                    Self::entry_damage(ctx, modifiers, damage_type, self.item_closures(item_id));

                (damage, damage_type == DamageType::Physical, false, false)
            }
            ComboStep::Rune(rune_id) => {
                let rune = rune_id.data();
                let closures = match self.attack_type {
                    AttackType::Melee => &rune.melee,
                    AttackType::Ranged => &rune.ranged,
                };

                let ctx = match self.data.rune_stacks.get(rune_id) {
                    Some(stacks) => &Ctx { stacks, ..*ctx },
                    None => ctx,
                };

                let damage_type = rune.metadata.damage_type;
                let damage = Self::entry_damage(ctx, modifiers, damage_type, closures);

                (damage, damage_type == DamageType::Physical, false, false)
            }
        }
    }

    /// Evaluates the expanded `steps` against `target`, writing the columns of
    /// [`ComboRow`] in `out`, which holds `steps.len() + ComboRow::SUMMARY` values
    pub fn eval(&self, steps: &[ComboStep], target: ComboTarget, out: &mut [i32]) {
        let ComboTarget {
            mut ctx,
            mut state,
            modifiers,
            shred,
            shadowflame_items,
            extra_item,
        } = target;

        debug_assert_eq!(out.len(), steps.len() + ComboRow::SUMMARY);

        // The row's modifiers already hold the health-gated multipliers at the
        // starting health, so each step applies the ratio to them
        let start_runes = state.rune_damage_modifier(self.owned_runes);
        let start_shadowflame = state.shadowflame_modifier(shadowflame_items);

        let guinsoo = self.owned_items.contains(&ItemId::GuinsoosRageblade);
        let cleaver = self.owned_items.contains(&ItemId::BlackCleaver);
        let base_armor = state.armor_values.modifier;

        let mut armor_ratio = 1.0;
        let mut carve = 0.0;
        let mut fight = FightState {
            stacks: ctx.stacks,
            counter: 0,
        };

        let mut health = state.current_stats.current_health;
        let mut total = 0.0;
        let mut kill_step = -1;
        let mut attacks = 0u32;

        for (i, &step) in steps.iter().enumerate() {
            // D5: formulas never see negative health
            let current_health = health.max(0.0);

            ctx.enemy_current_health = current_health;
            ctx.stacks = fight.stacks;
            state.current_stats.current_health = current_health;

            let mut modifiers = modifiers;
            let damages = &mut modifiers.damages;
            let shadowflame = state.shadowflame_modifier(shadowflame_items) / start_shadowflame;

            damages.global_mod *= state.rune_damage_modifier(self.owned_runes) / start_runes;
            damages.magic_mod *= shadowflame;
            damages.true_mod *= shadowflame;
            damages.physical_mod *= armor_ratio;

            // Parts of mixed values read them from the context
            let ctx = ctx.with_multipliers(modifiers.damages.multipliers());
            let (mut damage, physical, onhit, is_attack) = self.step_damage(step, &ctx, &modifiers);

            if onhit {
                let every_hit = self.sum_entries(&ctx, &modifiers, extra_item, Attrs::onhit_min);
                let seething_strike =
                    guinsoo && is_attack && (attacks + 1).is_multiple_of(SEETHING_STRIKE_ATTACKS);

                damage += every_hit
                    * match seething_strike {
                        true => 2.0,
                        false => 1.0,
                    };
            }

            if is_attack {
                attacks += 1;
            }

            health -= damage;
            total += damage;
            out[i] = damage as i32;

            if kill_step < 0 && health <= 0.0 {
                kill_step = i as i32;
            }

            // Carve: each physical hit reduces the target's armor for the next ones
            if cleaver && physical && damage > 0.0 && carve < items::black_cleaver::CARVE_MAX_STACKS
            {
                carve += 1.0;

                let armor_values = RiotFormulas::real_resist(
                    RiotFormulas::combine_percentage(
                        shred.armor_penetration_percent,
                        items::black_cleaver::CARVE_ARMOR_REDUCTION_PER_STACK * carve,
                    ),
                    shred.armor_penetration_flat,
                    state.current_stats.armor,
                    state.accept_negatives,
                );

                armor_ratio = armor_values.modifier / base_armor;
            }

            self.champion_id.after_step(step, damage, onhit, &mut fight);
        }

        let n = steps.len();

        out[n] = total as i32;
        out[n + 1] = health as i32;
        out[n + 2] = kill_step;
    }
}
