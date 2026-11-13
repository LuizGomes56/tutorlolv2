use super::*;

impl GeneratorV2 for Lucian {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take("Innate - Vigilance", P(_1))?
            .damage_type(P(_1), Magic)
            .take("Physical Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Physical Damage Per Shot", R(Min))?
            .derive(R(Max), R(Min), |dmg| {
                // 22 (+ 22.222 per 100% critical strike chance) shots, as its
                // description writes
                let shots = f![22 + 22.222 * CritChance / 100];
                /* Maybe include IE bonus shots in the future */
                f![(dmg) * shots]
            })?
            .comment(R(Max), "Maximum Damage without IE")
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Attack, Ability(Q(Void)), Attack],
                &[
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Attack, Ability(W(Void)), Attack],
                &[Attack, Ability(Q(Void)), Attack, Ability(W(Void)), Attack],
                &[Ability(Q(Void)), Attack, Attack],
            ])?
            .end()
    }
}
