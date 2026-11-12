use super::*;

impl GeneratorV2 for Ryze {
    fn generate(&mut self) -> MayFail {
        // Against a Fluxed enemy, Overload deals 10 / 40 / 70 / 100% increased damage by
        // Realm Warp's rank, 0 to 3, as Overload's page writes it. A rank 0 of 0% would
        // be a `set` of this formula
        let flux = self.value("Flux Bonus", Key::Q)?;

        self.take("Magic Damage", Q(Void))?
            .derive(Q(_1), Q(Void), |q| f![q * (1 + flux / 100)])?
            .comment(Q(_1), "Flux empowered damage")
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
