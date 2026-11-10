use super::*;

impl Generator for Shaco {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability(Key::Q, [("Bonus Physical Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Increased Damage", Max),
                    ("Magic Damage", Min),
                ],
            )
            .ability(
                Key::E,
                [
                    ("Increased Damage", Max),
                    ("Magic Damage", Min),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Increased Modified Damage", Max),
                    ("Magic Damage", Void),
                    ("Modified Magic Damage", Min),
                ],
            )
            .damage_type(Q(Void), Physical)?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(R(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[Ability(E(Min)), Ability(Q(Void)), Ability(E(Min)), Attack],
            ])?
            .end()
    }
}
