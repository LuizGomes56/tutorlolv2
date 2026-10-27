use super::*;

impl Generator for TahmKench {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, _1) /* Innate */, (2, _2) /* Innate [1] */])
            .modify(P(_2), |dmg| f![(dmg / 100 * AbilityPower)])?
            .merge_sum([P(_1), P(_2)], P(Void))?
            .ability(Key::Q, [(1, Min) /* Magic Damage */]);

        let passive = self.damage_of(Q(Min))?.clone();

        self.clone_with(Q(Min), Q(Max), |dmg| f![(dmg) + passive])?
            .comment(Q(Max), "Damage Including Passive")?
            .ability(Key::W, [(1, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
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
