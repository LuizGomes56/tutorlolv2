use super::*;

impl Generator for Vayne {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, Max), /* Bonus True Damage */
                    (1, Min), /* Minimum Bonus Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Void), /* Bonus Physical Damage */
                    (1, Min),  /* Physical Damage */
                    (2, Max),  /* Total Physical Damage */
                ],
            )
            .damage_types([Q(Void), E(Void), E(Min), E(Max)], Physical)?
            .combos([
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Ability(E(Void)), Attack, Attack],
            ])?
            .end()
    }
}
