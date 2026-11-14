use super::*;

impl Generator for Swain {
    fn generate(&mut self) -> MayFail {
        let [fragment_health] = self
            .description(Key::P, "Innate")?
            .capture(regex!(r"Swain gains (\d+) bonus health"))?;

        self.constant("SOUL_FRAGMENT_HEALTH", fragment_health)?
            .take("Bonus Damage Per Bolt", Q(Void))?
            .take("Magic Damage", Q(Min))?
            .take("Total Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage per Tick", R(Void))?;

        // Demonic Ascension ticks every 0.5 seconds
        let (mode, per_second) = self.map_value(Key::R, "Base damage per second")?;

        self.map_base(R(Void), mode, per_second.times(0.5))?
            .take("Magic Damage", R(_1))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(_1)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(R(_1)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(_1)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(R(_1)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
