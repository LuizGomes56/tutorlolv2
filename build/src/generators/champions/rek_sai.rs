use super::*;

impl Generator for RekSai {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(_1))?
            .take("Bonus Physical Damage", Q(Min))?
            .take("Total Bonus Physical Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Physical Damage", E(Void))?
            .take("True Damage", E(_1))?
            .take("Physical Damage", R(Void))?
            .comment(Q(_1), "Prey Seeker while burrowed")
            .damage_type(E(_1), True)
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Attack, Ability(Q(Max)), Attack, Attack, Ability(E(Void))],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
