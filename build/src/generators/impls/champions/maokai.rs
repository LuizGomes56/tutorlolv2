use super::*;

impl Generator for Maokai {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Magic Damage", Min),
                    ("Non-Minion Total Magic Damage", Max),
                ],
            )
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Ability(R(Void)), Attack],
                &[Ability(Q(Void)), Ability(W(Void)), Attack],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
