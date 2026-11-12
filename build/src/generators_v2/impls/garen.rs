use super::*;

impl GeneratorV2 for Garen {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Physical Damage Per Spin", E(Min))?
            .take("Critical Strike Damage Per Spin", E(_1))?
            .derive(E(Max), E(Min), |e| f![7 * (e)])?
            .comment(E(Max), "Total damage of 7 spins")
            .take("True Damage", R(Void))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(Q(Void)), Ability(E(Max)), Attack, Ability(R(Void))],
                &[Attack, Ability(Q(Void)), Ability(E(Max)), Attack],
                &[Ability(E(Max)), Attack, Ability(Q(Void)), Ability(R(Void))],
                &[Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
