use super::*;

impl Generator for Leona {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .ability(Key::Q, [(0, Void) /* Bonus Magic Damage */])
            .ability(Key::W, [(3, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
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
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack],
                &[Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
