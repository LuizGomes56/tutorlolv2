use super::*;

/// The abilities v1 edited are written with v1's formulas, to be read from the wiki
impl GeneratorV2 for Samira {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Min))?
            .set(P(Min), Formula::Raw("(2 + (19-2)/17*(ctx.level-1)) + (((0.035) + (0.004117647 * (ctx.level - 1.0))) * ctx.attack_damage)".into()))
            .damage_type(P(Min), Magic)
            .insert(
                P(_1Min),
                Damage {
                    formula: Formula::Raw("(0.17) * ctx.attack_damage".into()),
                    name: "Daredevil Impulse".into(),
                    damage_type: Magic,
                    comment: String::new(),
                    attributes: Undefined,
                },
            )?
            .insert(
                P(Max),
                Damage {
                    formula: Formula::Raw("((2 + (19-2)/17*(ctx.level-1)) + (((0.035) + (0.004117647 * (ctx.level - 1.0))) * ctx.attack_damage)) * 2".into()),
                    name: "Daredevil Impulse".into(),
                    damage_type: Magic,
                    comment: String::new(),
                    attributes: Undefined,
                },
            )?
            .insert(
                P(_1Max),
                Damage {
                    formula: Formula::Raw("((0.17) * ctx.attack_damage) * 6".into()),
                    name: "Daredevil Impulse".into(),
                    damage_type: Magic,
                    comment: String::new(),
                    attributes: Undefined,
                },
            )?
            .take("Physical Damage", Q(Void))?
            .take("Physical Damage per Hit", W(Min))?
            .take("Total Physical Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Physical Damage Per Shot", R(Min))?
            .take("Total Physical Damage", R(Max))?
            .comment(P(_1Min), "Passive against immobilized targets")
            .combos([
                &[
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(W(Max)), Attack],
            ])?
            .end()
    }
}
