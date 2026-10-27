use super::*;

impl Generator for Hwei {
    fn generate(&mut self) -> MayFail {
        // The wiki groups each spell under the key that completes it,
        // e.g. Severing Bolt (QW) is found under W
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .comment(Q(_1), "Devastating Fire (QQ)")?
            .ability_nth(3, Key::Q, [(1, _2) /* Magic Damage */])
            .comment(Q(_2), "Grim Visage (EQ)")?
            .ability(
                Key::W,
                [
                    (0, Min), /* Magic Damage */
                    (1, Max), /* Maximum Damage */
                ],
            )
            .comment(W(Min), "Severing Bolt (QW)")?
            .comment(W(Max), "Severing Bolt (QW) against low health targets")?
            .ability_nth(2, Key::W, [(0, _1) /* Magic Damage */])
            .comment(W(_1), "Gaze of the Abyss (EW)")?
            .ability_nth(2, Key::E, [(2, _1) /* Total Fissure Magic Damage */])
            .comment(E(_1), "Molten Fissure (QE)")?
            .ability_nth(1, Key::E, [(0, _2) /* Bonus Magic Damage */])
            .comment(E(_2), "Stirring Lights (WE)")?
            .ability(Key::E, [(0, _3) /* Magic Damage */])
            .comment(E(_3), "Crushing Maw (EE)")?
            .ability_nth(
                1,
                Key::R,
                [
                    (0, Min), /* Magic Damage */
                    (2, Max), /* Maximum Total Damage */
                ],
            )
            .combo([
                Ability(E(_1)),
                Ability(Q(_1)),
                Ability(R(Max)),
                Ability(W(Max)),
                Ability(E(_1)),
                Ability(Q(_1)),
                Ability(Q(_1)),
                Attack,
            ])?
            .combo([
                Ability(W(Max)),
                Ability(E(_1)),
                Ability(E(_1)),
                Ability(Q(_1)),
                Ability(R(Max)),
                Ability(Q(_1)),
                Ability(Q(_1)),
            ])?
            .combo([
                Ability(W(Max)),
                Ability(Q(_1)),
                Ability(R(Max)),
                Ability(E(_1)),
                Ability(Q(_1)),
                Ability(Q(_1)),
                Ability(Q(_1)),
            ])?
            .combo([
                Ability(E(_1)),
                Ability(W(Min)),
                Ability(W(Min)),
                Ability(E(_1)),
                Ability(Q(_1)),
                Ability(W(Min)),
                Ability(R(Max)),
            ])?
            .combo([
                Ability(W(Max)),
                Ability(E(_1)),
                Ability(E(_1)),
                Ability(Q(_1)),
                Ability(Q(_1)),
                Ability(Q(_1)),
            ])?
            .combo([
                Ability(R(Max)),
                Ability(Q(_1)),
                Ability(E(_1)),
                Ability(E(_1)),
                Ability(E(_1)),
            ])?
            .end()
    }
}
