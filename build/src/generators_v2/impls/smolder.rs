use super::*;

impl GeneratorV2 for Smolder {
    fn generate(&mut self) -> MayFail {
        // Bonus magic damage of each basic ability by Dragon Practice stacks, from the
        // passive's page: Q's grows with critical strike chance, as E's
        let q_passive = self.value_nth("Innate", 0, Key::P)?;
        let w_passive = self.value_nth("Innate", 1, Key::P)?;
        let e_passive = self.value_nth("Innate", 2, Key::P)?;

        // Q's physical damage increase by critical strike chance, its Tier 2 bolts'
        // share of the damage and its Tier 3 burn
        let critical = self.value("Active", Key::Q)?;
        let bolts = self.value("Tier 2 - 125 stacks", Key::Q)?;
        let burn = self.value_nth("Tier 3 - 225 stacks", 0, Key::Q)?;

        self.take_nth("Physical Damage", 0, Q(Min))?
            .edit(Q(Min), |q| {
                f![q * (1 + critical) * PhysicalMultiplier + q_passive * MagicMultiplier]
            })
            // With the 2 bolts of Tier 2, which deal a share of the same damage
            .derive(Q(Max), Q(Min), |q| f![q * (1 + 2 * bolts / 100) + burn])?
            .edit(Q(Min), |q| f![q + burn])
            .damage_type(Q(Min), Mixed)
            .damage_type(Q(Max), Mixed)
            .take("Explosion Physical Damage", W(Void))?
            .edit(W(Void), |w| {
                f![w * PhysicalMultiplier + w_passive * MagicMultiplier]
            })
            .damage_type(W(Void), Mixed)
            .take("Glob Physical Damage", W(Min))?
            .damage_type(W(Min), Physical)
            .take("Total Physical Damage On Champion Hit", W(Max))?
            .edit(W(Max), |w| {
                f![w * PhysicalMultiplier + w_passive * MagicMultiplier]
            })
            .damage_type(W(Max), Mixed)
            .take("Physical Damage per Hit", E(Min))?
            .edit(E(Min), |e| {
                f![e * PhysicalMultiplier + e_passive * MagicMultiplier]
            })
            // 5 (+ 1 per 100 stacks) bolts, as its description writes
            .derive(E(Max), E(Min), |e| f![e * (5 + Stacks / 100)])?
            .damage_type(E(Min), Mixed)
            .damage_type(E(Max), Mixed)
            .take("Physical Damage", R(Min))?
            .take("Increased Physical Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Max)),
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Attack, Ability(W(Void)), Attack, Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
