use super::*;

/// The abilities v1 edited are written with v1's formulas, to be read from the wiki
impl GeneratorV2 for Warwick {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take("Magic Damage", Q(Void))?
            .take("Total Magic Damage", R(Void))?
            .combos([
                &[Ability(Q(Void)), Attack, Ability(R(Void))],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(R(Void)), Attack, Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void)), Attack, Attack, Attack],
            ])?
            .end()
    }
}
