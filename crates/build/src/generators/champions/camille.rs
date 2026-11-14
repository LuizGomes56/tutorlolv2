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

        self.take("Bonus Physical Damage", Q(Min))?
            .take("Increased Mixed Damage", Q(Max))?
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Mixed)
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
