use super::*;

impl Generator for JarvanIV {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(Key::Q, [(1, Void) /* Physical Damage */])
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Physical Damage */])
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
