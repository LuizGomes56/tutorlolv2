use super::*;

impl Generator for KSante {
    fn generate(&mut self) -> MayFail {
        // "{{pp|2 + (2-1)/17*(x-1) for 20|key=%}}" of the target's maximum health, by
        // the position of its numbers: a start and its growth to level 18
        let (mode, numbers) = self.map_numbers(Key::P, "Health ratio")?;
        let [start, high, low, levels, ..] = numbers[..] else {
            return Err("[KSante] P: no growth in \"Health ratio\"".into());
        };

        let health = f![start / 100 + (high - low) / levels / 100 * (Level - 1)];

        self.take_nth("Innate", 0, P(Void))?
            .damage_type(P(Void), Physical)
            .comment(P(Void), "Mark consumption bonus damage")
            .map_ratio(P(Void), mode, &[EnemyMaxHealth], health)?
            .take("Physical Damage", Q(Void))?
            .take("Physical Damage", W(Min))?
            .take("Total Maximum Mixed Damage", W(Max))?
            .damage_type(W(Max), Mixed)
            .take("Strike Physical Damage", R(Min))?
            .take("Total Physical Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
