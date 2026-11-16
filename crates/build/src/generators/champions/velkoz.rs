use super::*;

impl Generator for Velkoz {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Magic Damage", Q(Void))?
            .take_nth("Magic Damage", 1, W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Damage Per Tick", R(Min))?
            .take("Maximum Damage", R(Max))?
            // The ray deals true damage to Researched targets only, which a table
            // can't know, so it is magic damage as on any other target
            .damage_type(R(Min), Magic)
            .damage_type(R(Max), Magic)
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
