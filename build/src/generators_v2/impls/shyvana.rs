use super::*;

impl GeneratorV2 for Shyvana {
    fn generate(&mut self) -> MayFail {
        self.take("Area Physical Damage", Q(Void))?
            .take("True Damage", Q(_1))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Subsequent Explosion Damage", E(Min))?
            .take("Increased/Explosion Magic Damage", E(Max))?
            .take("Magic Damage", R(Void))?
            .damage_type(Q(Void), Physical)
            .damage_type(Q(_1), True)
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
