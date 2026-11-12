use super::*;

impl GeneratorV2 for JarvanIV {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Physical Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[Ability(E(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(R(Void)), Attack],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Void))],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void))],
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack, Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
