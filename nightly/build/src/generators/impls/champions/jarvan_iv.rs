use super::*;

impl Generator for JarvanIV {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Physical Damage", Void)])
            .combos([
                &[Ability(E(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(R(Void)), Attack],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Void))],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void))],
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack, Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
