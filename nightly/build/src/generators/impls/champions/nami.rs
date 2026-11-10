use super::*;

impl Generator for Nami {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Magic Damage", Max),
                    ("Minimum Damage", Min),
                ],
            )
            .ability(
                Key::E,
                [
                    ("Bonus Magic Damage Per Hit", Min),
                    ("Total Bonus Magic Damage", Max),
                ],
            )
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(W(Max)), Ability(E(Max)), Attack],
                &[Ability(E(Max)), Ability(W(Max)), Attack],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
