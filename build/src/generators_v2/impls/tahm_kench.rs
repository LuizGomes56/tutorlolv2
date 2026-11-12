use super::*;

/// The abilities v1 edited are written with v1's formulas, to be read from the wiki
impl GeneratorV2 for TahmKench {
    fn generate(&mut self) -> MayFail {
        self.insert(
                P(Void),
                Damage {
                    formula: Formula::Raw("((match ctx.level {..=1 => 5.0, 2 => 10.0, 3 => 15.0, 4 => 20.0, 5 => 25.0, 6 => 30.0, 7 => 35.0, 8 => 40.0, 9 => 45.0, 10 => 50.0, 11 => 55.0, 12.. => 60.0}) + ((0.04) * ctx.bonus_health)) + (((0.0125) * ctx.bonus_health / 100 * ctx.ability_power))".into()),
                    name: "An Acquired Taste".into(),
                    damage_type: Magic,
                    comment: String::new(),
                    attributes: Undefined,
                },
            )?
            .take("Magic Damage", Q(Min))?
            // The model's Q already includes An Acquired Taste, which v1 added here
            .derive(Q(Max), Q(Min), |q| q)?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", R(Void))?
            .comment(Q(Max), "Damage Including Passive")
            .combos([
                &[Ability(R(Void)), Ability(W(Void))],
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                ],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[Attack, Attack, Attack, Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
