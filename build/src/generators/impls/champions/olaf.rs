use super::*;

impl Generator for Olaf {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* True Damage */])
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
