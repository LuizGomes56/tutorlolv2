use super::*;

impl GeneratorV2 for Renekton {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Min))?
            .take("Enhanced Damage", Q(Max))?
            .take("Physical Damage Per Hit", W(Min))?
            .take_nth("Total Physical Damage", 1, W(Max))?
            .take("Physical Damage", E(Min))?
            .take("Enhanced Physical Damage", E(_1Min))?
            .take("Total Physical Damage", E(Max))?
            .take("Total Enhanced Damage", E(_1Max))?
            .take("Magic Damage Per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[Attack, Ability(W(Max)), Attack],
                &[Ability(E(Min)), Attack, Ability(Q(Max)), Ability(E(Min))],
            ])?
            .end()
    }
}
