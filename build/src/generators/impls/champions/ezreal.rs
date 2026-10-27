use super::*;

impl Generator for Ezreal {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Bonus Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combos([
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
                &[Ability(Q(Void)), Ability(W(Void)), Ability(E(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
                &[Ability(W(Void)), Ability(E(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
