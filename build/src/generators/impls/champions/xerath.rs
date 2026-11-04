use super::*;

impl Generator for Xerath {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Increased Damage", Max),
                    ("Magic Damage", Min),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Increased Damage per Stack", Void),
                    ("Magic Damage", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .combos([
                &[Ability(E(Void)), Ability(W(Max)), Ability(Q(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(R(Void)),
                ],
                &[Ability(E(Void)), Ability(W(Max)), Ability(Q(Void))],
                &[Ability(W(Max)), Ability(Q(Void))],
                &[Ability(W(Max)), Ability(E(Void)), Ability(Q(Void))],
            ])?
            .end()
    }
}
