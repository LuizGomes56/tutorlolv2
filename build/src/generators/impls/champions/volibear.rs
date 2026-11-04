use super::*;

impl Generator for Volibear {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Lightning Claws", Void)])
            .ability(Key::Q, [("Bonus Physical Damage", Void)])
            .ability(Key::W, [("Physical Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Physical Damage", Void)])
            .damage_type(W(Void), Physical)?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(W(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
