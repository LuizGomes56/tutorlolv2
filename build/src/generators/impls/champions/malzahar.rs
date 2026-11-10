use super::*;

impl Generator for Malzahar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Magic Damage Per Tick", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Magic Damage Per Tick", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(E(Max)), Ability(Q(Void)), Ability(W(Void)), Attack],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
            ])?
            .end()
    }
}
