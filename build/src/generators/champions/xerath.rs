use super::*;

impl Generator for Xerath {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Min))?
            .take("Increased Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Increased Damage per Stack", R(Void))?
            .take("Magic Damage", R(Min))?
            .take("Total Magic Damage", R(Max))?
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
