use super::*;

impl Generator for Gangplank {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Innate */])
            .modify(P(Void), |_| f![50 + 200 / 17 * (Level - 1) + BonusAd])?
            .comment(P(Void), "Trial by Fire true damage over 2.5 seconds")?
            .ability(Key::Q, [(1, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* Bonus Champion Damage */])
            .ability(
                Key::R,
                [
                    (1, Min), /* Magic Damage Per Wave */
                    (3, Max), /* Total Magic Damage */
                    (4, _1),  /* Total Magic Damage with Fire at Will */
                    (6, _2),  /* True Damage with Death's Daughter */
                ],
            )
            .damage_type(R(_2), True)?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                ],
                &[Ability(E(Void)), Attack, Ability(E(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(E(Void))],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
