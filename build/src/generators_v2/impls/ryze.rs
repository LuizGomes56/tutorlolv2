use super::*;

impl GeneratorV2 for Ryze {
    fn generate(&mut self) -> MayFail {
        let r = self.value("Bonus Overload Damage", Key::R)?;

        self.take("Magic Damage", Q(Void))?
            .edit(Q(Void), |dmg| f![(dmg) * (1 + r / 100)])
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(E(Void)), Ability(Q(Void))],
                &[Ability(E(Void)), Ability(W(Void)), Attack, Ability(Q(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(Q(Void))],
            ])?
            .end()
    }
}
