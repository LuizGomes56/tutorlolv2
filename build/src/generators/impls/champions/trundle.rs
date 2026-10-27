use super::*;

impl Generator for Trundle {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, Void) /* Bonus Physical Damage */])
            .ability(
                Key::R,
                [
                    (2, Min),  /* Initial Magic Damage */
                    (3, Void), /* Magic Damage Per Second */
                    (5, Max),  /* Total Magic Damage */
                ],
            )
            .combos([
                &[Ability(R(Void)), Attack, Ability(Q(Void)), Attack],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void)), Attack, Attack, Attack],
            ])?
            .end()
    }
}
