use super::*;

impl GeneratorV2 for Seraphine {
    fn generate(&mut self) -> MayFail {
        self.take("Innate - Harmony", P(Void))?
            .take("Magic Damage", Q(Min))?
            .take("Maximum Enhanced Damage", Q(Max))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Max))],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Max)), Attack],
                &[Ability(E(Void)), Ability(R(Void)), Ability(Q(Max)), Attack],
                &[Ability(E(Void)), Ability(Q(Max)), Attack],
                &[
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
            ])?
            .end()
    }
}
