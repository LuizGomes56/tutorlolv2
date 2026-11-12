use super::*;

impl GeneratorV2 for Hwei {
    fn generate(&mut self) -> MayFail {
        self.take_in("Devastating Fire", "Magic Damage", Q(_1))?
            .take_in("Grim Visage", "Magic Damage", Q(_2))?
            .take_in("Gaze of the Abyss", "Magic Damage", W(_1))?
            .take_in("Severing Bolt", "Magic Damage", W(Min))?
            .take("Maximum Damage", W(Max))?
            .take("Total Fissure Magic Damage", E(_1))?
            .take("Bonus Magic Damage", E(_2))?
            .take_in("Crushing Maw", "Magic Damage", E(_3))?
            .take("Magic Damage", R(Min))?
            .take("Maximum Total Damage", R(Max))?
            .comment(Q(_1), "Devastating Fire (QQ)")
            .comment(Q(_2), "Grim Visage (EQ)")
            .comment(W(Min), "Severing Bolt (QW)")
            .comment(W(Max), "Severing Bolt (QW) against low health targets")
            .comment(W(_1), "Gaze of the Abyss (EW)")
            .comment(E(_1), "Molten Fissure (QE)")
            .comment(E(_2), "Stirring Lights (WE)")
            .comment(E(_3), "Crushing Maw (EE)")
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
