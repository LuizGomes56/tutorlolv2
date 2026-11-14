use super::*;

impl Generator for Kaisa {
    fn generate(&mut self) -> MayFail {
        let [missiles, subsequent] = self.description(Key::Q, "Active")?.capture(regex!(
            r"releases a swarm of (\d+) missiles.*?take ([\d.]+)\*100% damage from missiles beyond their first"
        ))?;

        let [evolved] = self
            .description(Key::Q, "Evolution")?
            .capture(regex!(r"instead fires (\d+) missiles"))?;

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
            // All missiles, and the evolved ones, on one target: the first one deals
            // full damage, the next ones a share of it
            .derive(Q(Max), Q(Min), |q| {
                q.scale(1.0 + (missiles - 1.0) * subsequent)
            })?
            .comment(Q(Max), "Total Single-Target Damage")
            .derive(Q(_1), Q(Min), |q| {
                q.scale(1.0 + (evolved - 1.0) * subsequent)
            })?
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
