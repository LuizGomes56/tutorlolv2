use super::*;

impl Generator for Kaisa {
    fn generate(&mut self) -> MayFail {
        let per_stack = self.value_nth("Innate - Caustic Wounds", 1, Key::P)?;

        // "On-hit damage changed to 50% value", by the position of its numbers
        let (mode, numbers) = self.map_numbers(Key::P, "On-hit damage")?;
        let [share, ..] = numbers[..] else {
            return Err("[Kaisa] P: no number in \"On-hit damage\"".into());
        };

        let on_hit = Formula::Const(1.0).with_map(mode, Formula::Const(share / 100.0));

        self.take_nth("Innate - Caustic Wounds", 0, P(Void))?
            .take_nth("Innate - Caustic Wounds", 3, P(_1))?
            .edit(P(Void), |p| f![on_hit * (p + Stacks * (per_stack))])
            .comment(
                P(Void),
                "Caustic Wounds on-hit damage, Stacks = Plasma stacks",
            )
            .comment(P(_1), "Plasma detonation at 5 stacks")
            .map_change(P(_1), "Detonation base damage")?
            .take("Physical Damage Per Missile", Q(Min))?
            .map_change(Q(Min), "Bonus AD ratio")?
            .map_change(Q(Min), "Bonus AD ratio per missile")?
            .map_change(Q(Min), "AP ratio per missile")?
            // All 6 missiles, and the 12 evolved ones, on one target: the first one deals
            // full damage, the next ones 25%
            .derive(Q(Max), Q(Min), |q| q.scale(2.25))?
            .comment(Q(Max), "Total Single-Target Damage")
            .derive(Q(_1), Q(Min), |q| q.scale(3.75))?
            .comment(Q(_1), "Total Evolved Single-Target Damage")
            .take("Magic Damage", W(Void))?
            .combos([
                &[Ability(W(Void)), Ability(Q(Max)), Attack, Attack, Attack],
                &[
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Max)), Attack, Attack],
                &[Attack, Ability(Q(Max)), Attack, Ability(W(Void)), Attack],
                &[Attack, Ability(W(Void)), Ability(Q(Max)), Attack, Attack],
            ])?
            .end()
    }
}
