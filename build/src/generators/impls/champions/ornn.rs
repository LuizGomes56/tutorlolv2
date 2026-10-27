use super::*;

impl Generator for Ornn {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(3, Void) /* Innate - Temper [1] */])
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Magic Damage Per Tick */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(
                Key::R,
                [
                    (0, Min), /* Magic Damage */
                    (2, Max), /* Total Magic Damage */
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[Ability(R(Min)), Ability(E(Void)), Ability(R(Min)), Attack],
                &[
                    Ability(R(Min)),
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack, Ability(E(Void)), Attack],
                &[Attack, Ability(W(Max)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(W(Max)), Attack],
            ])?
            .end()
    }
}
