use super::*;

impl Generator for Lux {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[Ability(E(Void)), Attack, Ability(E(Void))],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
