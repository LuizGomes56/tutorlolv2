use super::*;

impl Generator for Velkoz {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Description 1", Void)])
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Magic Damage", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Damage Per Tick", Min),
                    ("Maximum Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Min)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Min)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(W(Min)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(R(Max))],
                &[
                    Ability(E(Void)),
                    Ability(W(Min)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Ability(E(Void)),
                    Ability(W(Min)),
                ],
                &[Ability(E(Void)), Ability(W(Max))],
            ])?
            .end()
    }
}
