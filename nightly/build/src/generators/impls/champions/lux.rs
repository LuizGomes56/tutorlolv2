use super::*;

impl Generator for Lux {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Magic Damage", Void)])
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
