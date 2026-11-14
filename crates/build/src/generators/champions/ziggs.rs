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

        // Subsequent mines deal 40%, and at most 10 of them hit
        let (mode, first) = self.map_value(Key::E, "First hit AP ratio")?;

        self.map_ratio(E(Void), mode, &[AbilityPower], first.clone())?
            .map_ratio(E(Min), mode, &[AbilityPower], first.clone().times(0.4))?
            .map_ratio(E(Max), mode, &[AbilityPower], first.times(5.0))?
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
