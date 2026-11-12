use super::*;

impl GeneratorV2 for Vex {
    fn generate(&mut self) -> MayFail {
        // Gloom's detonation, then its damage against non-champions
        self.take_nth("Innate - Gloom", 0, P(Void))?
            .comment(P(Void), "Gloom detonation bonus magic damage")
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            // The cast and the recast
            .take_nth("Magic Damage", 0, R(Min))?
            .take_nth("Magic Damage", 1, R(Max))?
            .combos([
                &[
                    Ability(R(Min)),
                    Ability(R(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Ability(R(Min)),
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Min)),
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Min)),
                    Ability(R(Min)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Min)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
