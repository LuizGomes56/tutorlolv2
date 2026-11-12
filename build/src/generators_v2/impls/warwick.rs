use super::*;

/// The abilities v1 edited are written with v1's formulas, to be read from the wiki
impl GeneratorV2 for Warwick {
    fn generate(&mut self) -> MayFail {
        self.insert(
                P(Void),
                Damage {
                    formula: Formula::Raw("((6 + (55-6)/17*(ctx.level-1)) + ((0.15) * ctx.bonus_ad)) + ((0.1) * ctx.ability_power)".into()),
                    name: "Eternal Hunger".into(),
                    damage_type: Magic,
                    comment: String::new(),
                    attributes: Undefined,
                },
            )?
            .take("Magic Damage", Q(Void))?
            .take("Total Magic Damage", R(Void))?
            .combos([
                &[Ability(Q(Void)), Attack, Ability(R(Void))],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(R(Void)), Attack, Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void)), Attack, Attack, Attack],
            ])?
            .end()
    }
}
