use super::*;

impl Generator for Ziggs {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .map_change(P(Void), "AP ratio")?
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage per Mine", E(Void))?
            .take("Reduced Damage per Mine", E(Min))?
            .take("Maximum Total Magic Damage", E(Max))?
            .take("Reduced Damage", R(Min))?
            .take("Epicenter Magic Damage", R(Max))?;

        // Subsequent mines deal a share of the first, and every mine of the cluster hits
        let [mines, subsequent] = self.description(Key::E, "Active")?.capture(regex!(
            r"scatters a cluster of (\d+) proximity mines.*?takes ([\d.]+)% damage from subsequent mines"
        ))?;

        let subsequent = subsequent / 100.0;
        let (mode, first) = self.map_value(Key::E, "First hit AP ratio")?;

        self.map_ratio(E(Void), mode, &[AbilityPower], first.clone())?
            .map_ratio(
                E(Min),
                mode,
                &[AbilityPower],
                first.clone().times(subsequent),
            )?
            .map_ratio(
                E(Max),
                mode,
                &[AbilityPower],
                first.times(1.0 + (mines - 1.0) * subsequent),
            )?
            .combos([
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[Ability(W(Void)), Ability(W(Void)), Ability(Q(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                ],
            ])?
            .end()
    }
}
