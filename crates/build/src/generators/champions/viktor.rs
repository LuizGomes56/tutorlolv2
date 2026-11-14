use super::*;

impl Generator for Viktor {
    fn generate(&mut self) -> MayFail {
        // Death Ray's laser, which deals damage before its aftershock
        let laser = self.value_nth("Magic Damage", 0, Key::E)?;
        let laser = self.map_changed(Key::E, "Laser AP ratio", laser)?;

        self.take("Modified Magic Damage", Q(Void))?
            .map_change(Q(Void), "Discharge base damage")?
            .map_change(Q(Void), "Discharge AP ratio")?
            .take("Magic Damage", Q(Min))?
            .map_change(Q(Min), "Device base damage")?
            .map_change(Q(Min), "Device AP ratio")?
            .take("Total Magic Damage", Q(Max))?
            .take_nth("Magic Damage", 1, E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Magic Damage", R(Void))?
            .map_change(R(Void), "Initial base damage")?
            .take("Magic Damage Per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?;

        let device = self.damage(Q(Min))?;
        let discharge = self.damage(Q(Void))?;
        let aftershock = self.damage(E(Min))?;
        let initial = self.damage(R(Void))?;
        let tick = self.damage(R(Min))?;

        self.set(Q(Max), f![device + discharge])
            .set(E(Max), f![laser + aftershock])
            // The initial hit and 6 ticks
            .set(R(Max), f![initial + 6 * tick])
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Ability(E(Max))],
            ])?
            .end()
    }
}
