use super::*;

impl Generator for Nasus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(
                Key::E,
                [
                    (1, Min), /* Magic Damage */
                    (2, _1),  /* Magic Damage Per Tick */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (3, Min), /* Magic Damage Per Tick */
                    (4, Max), /* Total Magic Damage */
                ],
            )
            .combos([
                &[
                    Ability(E(Min)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[Ability(E(Min)), Ability(Q(Void)), Ability(E(Min)), Attack],
                &[Ability(E(Max)), Attack, Ability(Q(Void))],
                &[Attack, Ability(Q(Void)), Ability(E(Max)), Attack],
            ])?
            .end()
    }
}
