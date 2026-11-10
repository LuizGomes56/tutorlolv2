use super::*;

impl Generator for Quinn {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Description 1", Void)])
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(Key::E, [("Physical damage", Void)])
            .ability(Key::R, [("Physical Damage", Void)])
            .combos([
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Ability(Q(Void))],
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
