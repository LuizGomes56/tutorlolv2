use super::*;

impl Generator for Kindred {
    fn generate(&mut self) -> MayFail {
        // A value written in a shape map changes can't read, by the position of its
        // numbers, as {{fd|3}}% of "3% per 100 bonus AD"
        let (mode, numbers) = self.map_numbers(Key::W, "Damage per mark")?;
        let [per_mark, ..] = numbers[..] else {
            return Err("[Kindred] W: no number in \"Damage per mark\"".into());
        };

        self.take("Physical Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .map_change(W(Void), "Base damage")?
            .map_change(W(Void), "Bonus AD ratio")?
            .map_change(W(Void), "Health ratio")?
            .map_ratio(
                W(Void),
                mode,
                &[EnemyCurrentHealth, Stacks],
                per_mark / 100.0,
            )?
            .take("Additional Physical Damage", E(Void))?
            .map_change(E(Void), "Base health ratio")?
            .combos([
                &[
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
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
