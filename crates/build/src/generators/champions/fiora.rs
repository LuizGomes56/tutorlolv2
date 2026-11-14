use super::*;

impl Generator for Fiora {
    fn generate(&mut self) -> MayFail {
        // A value written in a shape map changes can't read, by the position of its
        // numbers, as {{fd|3}}% of "3% per 100 bonus AD"
        let (mode, numbers) = self.map_numbers(Key::P, "Bonus AD ratio")?;
        let [per_100, ..] = numbers[..] else {
            return Err("[Fiora] P: no number in \"Bonus AD ratio\"".into());
        };

        self.take_nth("Innate", 0, P(Void))?
            .comment(P(Void), "Vital true damage")
            .map_change(P(Void), "Base damage")?
            .map_ratio(P(Void), mode, &[EnemyMaxHealth, BonusAd], per_100 / 100.0)?
            .take("Physical Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Critical damage", E(Void))?
            // A critical strike of 160 / 170 / 180 / 190 / 200% of AD
            .edit(E(Void), |e| f![e / 100 * AttackDamage])
            .comment(E(Void), "Second basic attack critical strike")
            .derive(R(Void), P(Void), |p| f![4 * (p)])?
            .comment(R(Void), "All 4 Vitals")
            .combos([
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
