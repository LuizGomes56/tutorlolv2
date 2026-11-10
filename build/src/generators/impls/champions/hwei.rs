use super::*;

impl Generator for Hwei {
    fn generate(&mut self) -> MayFail {
        // The wiki groups each spell under the key that completes it,
        // e.g. Severing Bolt (QW) is found under W
        self.ability(Key::Q, [("Magic Damage", _1)])
            .comment(Q(_1), "Devastating Fire (QQ)")?
            .ability_nth(3, Key::Q, [("Magic Damage", _2)])
            .comment(Q(_2), "Grim Visage (EQ)")?
            .ability(
                Key::W,
                [
                    ("Magic Damage", Min),
                    ("Maximum Damage", Max),
                ],
            )
            .comment(W(Min), "Severing Bolt (QW)")?
            .comment(W(Max), "Severing Bolt (QW) against low health targets")?
            .ability_nth(2, Key::W, [("Magic Damage", _1)])
            .comment(W(_1), "Gaze of the Abyss (EW)")?
            .ability_nth(2, Key::E, [("Total Fissure Magic Damage", _1)])
            .comment(E(_1), "Molten Fissure (QE)")?
            .ability_nth(1, Key::E, [("Bonus Magic Damage", _2)])
            .comment(E(_2), "Stirring Lights (WE)")?
            .ability(Key::E, [("Magic Damage", _3)])
            .comment(E(_3), "Crushing Maw (EE)")?
            .ability_nth(
                1,
                Key::R,
                [
                    ("Magic Damage", Min),
                    ("Maximum Total Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(E(_1)),
                    Ability(Q(_1)),
                    Ability(R(Max)),
                    Ability(W(Max)),
                    Ability(E(_1)),
                    Ability(Q(_1)),
                    Ability(Q(_1)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Ability(E(_1)),
                    Ability(E(_1)),
                    Ability(Q(_1)),
                    Ability(R(Max)),
                    Ability(Q(_1)),
                    Ability(Q(_1)),
                ],
                &[
                    Ability(W(Max)),
                    Ability(Q(_1)),
                    Ability(R(Max)),
                    Ability(E(_1)),
                    Ability(Q(_1)),
                    Ability(Q(_1)),
                    Ability(Q(_1)),
                ],
                &[
                    Ability(E(_1)),
                    Ability(W(Min)),
                    Ability(W(Min)),
                    Ability(E(_1)),
                    Ability(Q(_1)),
                    Ability(W(Min)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(W(Max)),
                    Ability(E(_1)),
                    Ability(E(_1)),
                    Ability(Q(_1)),
                    Ability(Q(_1)),
                    Ability(Q(_1)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(_1)),
                    Ability(E(_1)),
                    Ability(E(_1)),
                    Ability(E(_1)),
                ],
            ])?
            .end()
    }
}
