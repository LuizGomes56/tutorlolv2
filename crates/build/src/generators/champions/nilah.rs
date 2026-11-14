use super::*;

impl Generator for Nilah {
    fn generate(&mut self) -> MayFail {
        // Armor penetration grows linearly with critical strike chance, up to
        // its maximum at 100% critical strike chance
        let [min, max] = self.description(Key::Q, "Passive")?.capture(regex!(
            r"Nilah gains\s+([\d.]+) to ([\d.]+) for \d+ armor penetration"
        ))?;

        if min != 0.0 {
            return Err("[Nilah] Passive armor penetration was expected to start at 0".into());
        }

        self.constant("FORMLESS_BLADE_ARMOR_PENETRATION_PER_CRIT", max / 100.0)?;

        self.take("Minimum Physical Damage", Q(Min))?
            .take("Maximum Physical Damage", Q(Max))?
            .take("Physical Damage", E(Void))?
            .take("Maximum Total Physical Damage", R(_1))?
            .take("Burst Physical Damage", R(_2))?
            .take("Physical Damage per Tick", R(Min))?
            .take("Total Physical Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[Ability(E(Void)), Ability(R(Max))],
                &[Ability(E(Void)), Ability(R(Max)), Ability(E(Void))],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Min)),
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Attack, Attack, Attack],
            ])?
            .end()
    }
}
