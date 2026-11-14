use super::*;

impl Generator for Shen {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Magic Damage", Q(Min))?
            .take("Increased Bonus Damage", Q(_1Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Total Increased Damage", Q(_1Max))?
            .take("Physical Damage", E(Void))?
            .combos([
                &[Ability(E(Void)), Ability(Q(Max)), Attack, Attack, Attack],
                &[
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Max)), Attack, Attack, Attack],
                &[Ability(Q(Max)), Attack, Ability(E(Void))],
                &[Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
