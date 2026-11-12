use super::*;

impl GeneratorV2 for Qiyana {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take_in("Edge of Ixtal", "Physical Damage", Q(Min))?
            .take_in("Edge of Ixtal", "Reduced Damage", Q(_1))?
            .take("Increased Damage", Q(Max))?
            .take_in("Elemental Wrath", "Physical Damage", Q(_2))?
            .take("Subsequent Increased Damage", Q(_3))?
            .take("Bonus Magic Damage", W(Void))?
            .take("Physical Damage", E(Void))?
            .take("Physical Damage", R(Void))?
            .damage_type(P(Void), Physical)
            .damage_type(E(Void), Physical)
            .damage_type(R(Void), Physical)
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(E(Void)), Ability(Q(Max)), Attack],
                &[Ability(Q(Min)), Ability(W(Void)), Ability(Q(Min))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
            ])?
            .end()
    }
}
