use super::*;

impl Generator for Belveth {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Minimum Physical Damage per hit", E(Min))?
            .take("Maximum Physical Damage per hit", E(Max))?
            .take("True Damage", R(Void))?
            .take("Bonus True Damage", R(_1))?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Max)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Max)),
                ],
            ])?
            .end()
    }
}
