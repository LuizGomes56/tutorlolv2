use super::*;

/// The abilities v1 edited are written with v1's formulas, to be read from the wiki
impl GeneratorV2 for Smolder {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Physical Damage", 1, Q(Min))?
            .set(Q(Min), Formula::Raw("((((30.0) + (5.0 * (ctx.q_level - 1.0))) + ((0.65) * ctx.bonus_ad)) * ctx.physical_multiplier) + ((0.25) + (0.35 * (ctx.crit_chance * 0.01)) * ctx.stacks * ctx.magic_multiplier) + (0.083 * ctx.bonus_ad / 100 + 0.017 * ctx.stacks / 100) * ctx.enemy_max_health".into()))
            .damage_type(Q(Min), Mixed)
            .insert(
                Q(Max),
                Damage {
                    formula: Formula::Raw("2 * (((((30.0) + (5.0 * (ctx.q_level - 1.0))) + ((0.65) * ctx.bonus_ad)) * ctx.physical_multiplier) + ((0.25) + (0.35 * (ctx.crit_chance * 0.01)) * ctx.stacks * ctx.magic_multiplier) + (0.083 * ctx.bonus_ad / 100 + 0.017 * ctx.stacks / 100) * ctx.enemy_max_health)".into()),
                    name: "Super Scorcher Breath".into(),
                    damage_type: Mixed,
                    comment: String::new(),
                    attributes: Undefined,
                },
            )?
            .take("Explosion Physical Damage", W(Void))?
            .set(W(Void), Formula::Raw("(((((25.0) + (25.0 * (ctx.w_level - 1.0))) + ((0.5) * ctx.bonus_ad)) + ((0.8) * ctx.ability_power)) * ctx.physical_multiplier) + (0.08) * ctx.crit_chance * ctx.magic_multiplier".into()))
            .damage_type(W(Void), Mixed)
            .take("Glob Physical Damage", W(Min))?
            .set(W(Min), Formula::Raw("((((60.0) + (10.0 * (ctx.w_level - 1.0))) + ((0.6) * ctx.bonus_ad)) * ctx.physical_multiplier) + (0.08) * ctx.crit_chance * ctx.magic_multiplier".into()))
            .damage_type(W(Min), Mixed)
            .take("Total Physical Damage On Champion Hit", W(Max))?
            .set(W(Max), Formula::Raw("(((((85.0) + (35.0 * (ctx.w_level - 1.0))) + ((1.1) * ctx.bonus_ad)) + ((0.8) * ctx.ability_power)) * ctx.physical_multiplier) + (0.08) * ctx.crit_chance * ctx.magic_multiplier".into()))
            .damage_type(W(Max), Mixed)
            .insert(
                E(Min),
                Damage {
                    formula: Formula::Raw("(10 + 5 * ctx.e_level + 0.3 * ctx.attack_damage) * ctx.physical_multiplier + ((0.55) * ctx.stacks * ctx.magic_multiplier)".into()),
                    name: "Flap, Flap, Flap".into(),
                    damage_type: Mixed,
                    comment: String::new(),
                    attributes: Undefined,
                },
            )?
            .insert(
                E(Max),
                Damage {
                    formula: Formula::Raw("((10 + 5 * ctx.e_level + 0.3 * ctx.attack_damage) * ctx.physical_multiplier + ((0.55) * ctx.stacks * ctx.magic_multiplier)) * (5 + ctx.stacks / 100)".into()),
                    name: "Flap, Flap, Flap".into(),
                    damage_type: Mixed,
                    comment: String::new(),
                    attributes: Undefined,
                },
            )?
            .take("Physical Damage", R(Min))?
            .take("Increased Physical Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Max)),
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Attack, Ability(W(Void)), Attack, Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
