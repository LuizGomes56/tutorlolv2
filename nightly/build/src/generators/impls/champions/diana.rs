use super::*;

impl Generator for Diana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate - Moonsilver Blade", Void)])
            .ability(Key::Q, [("Magic Damage", _1)])
            .ability(
                Key::W,
                [
                    ("Magic Damage per Orb", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Bonus Damage Per Champion", _1),
                    ("Magic Damage", Min),
                    ("Total Damage Vs. 5 Champions", Max),
                ],
            )
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(Q(_1)),
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(Q(_1)),
                    Attack,
                ],
                &[
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(_1)),
                    Attack,
                ],
                &[
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(W(Max)), Ability(E(Void)), Ability(Q(_1))],
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Max)),
                    Ability(Q(_1)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Void)),
                ],
                &[Attack, Ability(W(Max)), Ability(Q(_1)), Attack, Attack],
            ])?
            .end()
    }
}
