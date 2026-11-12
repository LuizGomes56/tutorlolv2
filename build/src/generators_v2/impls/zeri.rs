use super::*;

/// The abilities v1 edited are written with v1's formulas, to be read from the wiki
impl GeneratorV2 for Zeri {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Min))?
            .take_nth("Innate", 2, P(Max))?
            .set(P(Max), Formula::Raw("((75 + (160-75)/17*(ctx.level-1)) + ((1.1) * ctx.ability_power)) + (((0.01) + (0.005882353 * (ctx.level - 1.0))) * ctx.enemy_max_health)".into()))
            .damage_type(P(Max), Magic)
            .take("Total Critical Strike Damage", Q(_1))?
            .take("Physical Damage per Hit", Q(Min))?
            .take("Total Physical Damage", Q(Max))?
            .take("Physical Damage", W(Void))?
            .take("Critical Strike Damage", W(_1))?
            .take("Burst Fire Bonus Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .damage_type(R(Void), Magic)
            .comment(P(Min), "Uncharged basic attack")
            .comment(P(Max), "Fully charged basic attack")
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(Q(Max)),
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(Q(Max)),
                ],
            ])?
            .end()
    }
}
