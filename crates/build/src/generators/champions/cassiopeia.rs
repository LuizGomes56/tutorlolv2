use super::*;

impl Generator for Cassiopeia {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage Per Tick", Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Magic Damage Per Second", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Bonus Magic Damage", E(Min))?
            .map_change(E(Min), "Poisoned damage AP ratio")?
            .take("Total Enhanced Damage", E(Max))?;

        // Twin Fang's own 20% AP, and its poisoned bonus
        let (mode, poisoned) = self.map_value(Key::E, "Poisoned damage AP ratio")?;
        let Formula::Const(poisoned) = poisoned else {
            return Err("[Cassiopeia] E: poisoned AP ratio by rank".into());
        };

        self.map_ratio(E(Max), mode, &[AbilityPower], 0.2 + poisoned)?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Ability(R(Void)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Ability(E(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                ],
            ])?
            .end()
    }
}
