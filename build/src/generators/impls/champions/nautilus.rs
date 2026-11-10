use super::*;

impl Generator for Nautilus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Magic Damage per Instance", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(
                Key::E,
                [
                    ("Magic Damage", Min),
                    ("Maximum Total Damage", Max),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Increased Damage", Max),
                    ("Magic Damage", Min),
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                ],
                &[Attack, Ability(Q(Void)), Ability(E(Max))],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
