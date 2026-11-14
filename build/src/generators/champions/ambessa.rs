use super::*;

impl Generator for Ambessa {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Medarda Maxim", 0, P(Void))?
            .take_in("Cunning Sweep", "Physical Damage", Q(Min))?
            .take_in("Sundering Slam", "Physical Damage", Q(_1Min))?
            .take_in("Cunning Sweep", "Increased Physical Damage", Q(Max))?
            .take_in("Sundering Slam", "Increased Physical Damage", Q(_1Max))?
            .take("Physical Damage", W(Min))?
            .take("Increased Physical Damage", W(Max))?
            .take("Physical Damage", E(Min))?
            .take("Total Physical Damage", E(Max))?
            .take("Physical Damage", R(Void))?
            .damage_type(P(Void), Physical)
            .combos([
                &[
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Max)),
                ],
            ])?
            .end()
    }
}
