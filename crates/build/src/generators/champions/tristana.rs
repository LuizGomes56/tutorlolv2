use super::*;

impl Generator for Tristana {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Minimum Physical Damage", E(Min))?
            .map_change(E(Min), "Minimum bonus AD ratio")?
            .map_change(E(Min), "Bomb AP ratio")?
            .take("Full Stack Physical Damage", E(Max))?
            .map_change(E(Max), "Maximum bomb AP ratio")?;

        // Each stack increases the charge's minimum, up to a number of stacks
        let [per_stack, stacks] = self
            .description(Key::E, "Active")?
            .capture(regex!(r"damage by ([\d.]+)\*100%, stack up to (\d+) times"))?;

        let (mode, minimum) = self.map_value(Key::E, "Minimum bonus AD ratio")?;

        self.map_ratio(
            E(Max),
            mode,
            &[BonusAd],
            minimum.times(1.0 + per_stack * stacks),
        )?
        .take("Magic Damage", R(Void))?
        .comment(E(Void), "Passive explosion damage on kill")
        .combos([
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(R(Void)),
                Attack,
                Attack,
                Ability(W(Void)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(R(Void)),
                Attack,
                Attack,
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(W(Void)),
            ],
            &[Attack, Ability(E(Void)), Attack, Attack, Attack, Attack],
        ])?
        .end()
    }
}
