use super::*;

impl Generator for Lux {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[Ability(E(Void)), Attack, Ability(E(Void))],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
