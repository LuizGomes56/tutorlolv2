use super::*;

impl Generator for Vex {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Description 2", Void)])
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Magic Damage", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(R(Min)),
                    Ability(R(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Ability(R(Min)),
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Min)),
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Min)),
                    Ability(R(Min)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Min)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
