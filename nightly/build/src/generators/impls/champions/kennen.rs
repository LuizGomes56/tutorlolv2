use super::*;

impl Generator for Kennen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Bonus Magic Damage", _1),
                    ("Magic Damage", Void),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Magic Damage Per Bolt", Min),
                    ("Total Single-Target Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Ability(W(Void))],
                &[Attack, Ability(Q(Void)), Attack, Ability(W(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
