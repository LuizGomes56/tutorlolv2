use super::*;

impl Generator for Ryze {
    fn generate(&mut self) -> MayFail {
        // Against a Fluxed enemy, Overload deals 10 / 40 / 70 / 100% increased damage by
        // Realm Warp's rank, 0 to 3, as Overload's page writes it. A rank 0 of 0% would
        // be a `set` of this formula
        let flux = self.value("Flux Bonus", Key::Q)?;

        // Arena's "Spell Flux bonus damage changed to {{pp|type=… 20px …|10 to 70 for
        // 4|0 to 3}}" by Realm Warp's rank, by the position of its numbers
        let (mode, numbers) = self.map_numbers(Key::Q, "{{ai|Spell Flux|Ryze}} bonus damage")?;
        let [_, first, last, ranks, ..] = numbers[..] else {
            return Err("[Ryze] Q: no values in its Spell Flux change".into());
        };

        let flux = flux.with_map(mode, f![first + (last - first) / (ranks - 1) * RLevel]);

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
