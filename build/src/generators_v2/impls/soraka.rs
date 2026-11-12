use super::*;

impl GeneratorV2 for Soraka {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .combos([
                &[Ability(Q(Void)), Ability(E(Max)), Attack, Attack],
                &[Ability(E(Max)), Ability(Q(Void))],
                &[Ability(E(Max)), Ability(Q(Void)), Attack, Attack],
                &[Ability(Q(Void)), Attack, Ability(E(Max))],
            ])?
            .end()
    }
}
