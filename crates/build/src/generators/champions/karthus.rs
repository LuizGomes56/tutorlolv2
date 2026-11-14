use super::*;

impl Generator for Karthus {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Min))?
            .take("Isolated Enhanced Damage", Q(Max))?
            .take("Magic Damage Per Tick", E(Min))?
            .take("Damage Per Second", E(Max))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                ],
                &[Ability(Q(Max)), Ability(R(Void))],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                ],
            ])?
            .end()
    }
}
