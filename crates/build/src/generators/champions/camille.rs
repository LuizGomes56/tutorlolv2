use super::*;

impl Generator for Camille {
    fn generate(&mut self) -> MayFail {
        // A value written in a shape map changes can't read, by the position of its
        // numbers, as {{fd|3}}% of "3% per 100 bonus AD"
        let (mode, numbers) = self.map_numbers(Key::W, "Outer cone bonus AD ratio")?;
        let [outer, ..] = numbers[..] else {
            return Err("[Camille] W: no number in \"Outer cone bonus AD ratio\"".into());
        };

        let outer_cone = self.value("Outer Cone Bonus Damage", Key::W)?;

        // Recast: "40 to 100 for 16 of the attack's total damage" is dealt as true
        // damage, growing evenly by level until level 16, and kept from there to 18.
        // Without the levels after the cap, the table would become a growth that
        // passes 100%
        let [low, high, cap] = self.description(Key::Q, "Recast")?.capture(regex!(
            r"(\d+) to (\d+) for (\d+) of the attack's total damage"
        ))?;

        let cap = cap as u8;
        let step = (high - low) / f64::from(cap - 1);
        let true_share = Formula::Table {
            var: Level,
            keys: (1..=wiki::monsters::MAX_LEVEL).collect(),
            values: (0..wiki::monsters::MAX_LEVEL)
                .map(|i| (low + step * f64::from(i.min(cap - 1))) / 100.0)
                .collect(),
        };

        self.take("Bonus Physical Damage", Q(Min))?
            .take("Increased Mixed Damage", Q(Max))?
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Mixed)
            // The doubled bonus rides on a basic attack, whose total is then split
            .edit(Q(Max), |q| {
                let total = f![AttackDamage + (q)];
                f![(total) * (1 - (true_share)) * PhysicalMultiplier
                    + (total) * (true_share) * TrueMultiplier]
            })
            .take("Physical Damage", W(Min))?
            .derive(W(Max), W(Min), |w| f![w + outer_cone])?
            .comment(W(Max), "Outer cone damage")
            .map_ratio(W(Max), mode, &[EnemyMaxHealth, BonusAd], outer / 100.0)?
            .take("Physical Damage", E(Void))?
            .take("Bonus Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Max)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Max)),
                ],
                &[Ability(E(Void)), Ability(E(Void))],
            ])?
            .end()
    }
}
