use super::*;

impl Generator for Yunara {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(3, Void) /* Passive Bonus Magic Damage */])
            .damage_type(Q(Void), Magic)?
            .ability_nth(
                1,
                Key::W,
                [
                    (0, Void), /* Initial Magic Damage */
                    (1, _1),   /* Linger Magic Damage per Tick */
                    (2, _2),   /* Total Expanded Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Arc of Ruin Base Damage */])
            .damage_type(R(Void), Magic)?
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
