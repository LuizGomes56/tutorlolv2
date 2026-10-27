use super::*;

impl Generator for Darius {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Min) /* Hemorrhage */])
            .modify(P(Min), |_| f![13 + (Level - 1) + 0.3 * BonusAd])?
            .comment(P(Min), "Hemorrhage total damage per stack")?
            .clone_with(P(Min), P(Max), |p| f![5 * (p)])?
            .comment(P(Max), "Hemorrhage total damage at 5 stacks")?
            .ability(
                Key::Q,
                [
                    (0, Max), /* Physical Damage (Blade) */
                    (1, Min), /* Reduced Damage (Handle) */
                ],
            )
            .ability(Key::W, [(0, Void) /* Bonus Physical Damage */])
            .ability(
                Key::R,
                [
                    (2, Min), /* True Damage */
                    (1, Max), /* Maximum True Damage */
                ],
            )
            .combos([
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Ability(R(Min)),
                ],
                &[Ability(Q(Max)), Attack, Ability(W(Void)), Ability(R(Max))],
                &[Attack, Ability(W(Void)), Ability(Q(Max))],
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Max))],
                &[Ability(Q(Max)), Attack, Ability(W(Void))],
            ])?
            .end()
    }
}
