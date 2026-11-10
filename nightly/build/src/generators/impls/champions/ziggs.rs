use super::*;

impl Generator for Ziggs {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Magic Damage per Mine", Void),
                    ("Maximum Total Magic Damage", Max),
                    ("Reduced Damage per Mine", Min),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Epicenter Magic Damage", Max),
                    ("Reduced Damage", Min),
                ],
            )
            .combos([
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[Ability(W(Void)), Ability(W(Void)), Ability(Q(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                ],
            ])?
            .end()
    }
}
