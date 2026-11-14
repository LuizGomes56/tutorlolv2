use super::*;

impl Generator for Jayce {
    fn generate(&mut self) -> MayFail {
        self.take_in("To the Skies!", "Physical Damage", Q(_1))?
            .take_in("Shock Blast", "Physical Damage", Q(Min))?
            .take("Increased Damage", Q(Max))?
            .take("Magic Damage Per Tick", W(Min))?
            .take("Physical Damage", W(_1Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Total Physical Damage", W(_1Max))?
            .take("Magic Damage", E(Void))?
            .comment(Q(Min), "Shock Blast")
            .comment(Q(Max), "Shock Blast through Acceleration Gate")
            .comment(Q(_1), "To the Skies!")
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Physical)
            .damage_type(Q(_1), Physical)
            .damage_type(W(_1Min), Physical)
            .damage_type(W(_1Max), Physical)
            .combos([
                &[Ability(W(Max)), Attack, Attack, Ability(E(Void)), Attack],
                &[Ability(Q(Max)), Attack, Ability(E(Void))],
                &[
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(Q(Min)),
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
