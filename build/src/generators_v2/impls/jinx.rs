use super::*;

impl GeneratorV2 for Jinx {
    fn generate(&mut self) -> MayFail {
        self.take("Attack Speed per Subsequent Stack", Q(Void))?
            .set(Q(Void), f![1.1 * AttackDamage])
            .take("Physical Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Maximum Physical Damage", R(Max))?
            .take("Maximum Secondary Damage", R(_1Max))?
            .take("Minimum Physical Damage", R(Min))?
            .take("Minimum Secondary Damage", R(_1Min))?
            .combos([
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(W(Void)), Attack, Attack, Attack],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
