use super::*;

impl Generator for Naafiri {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Damage {
                name: "Packmate".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Pet: Packmate basic attack".into(),
                formula: f![10 + 10 / 17 * (Level - 1) + 0.04 * BonusAd],
            },
        )?
        .map_change(P(Void), "Packmate attack damage bonus AD ratio")?
        .take("Minimum Total Physical Damage", Q(Min))?
        .take("Maximum Total Physical Damage", Q(Max))?
        .take("Total Bleed Physical Damage", Q(_1))?
        .map_change(Q(_1), "Bleed base damage")?
        .take("Dash Physical Damage", E(Min))?
        .map_change(E(Min), "Dash base damage")?
        .map_change(E(Min), "Dash AD ratio")?
        .take("Total Physical Damage", E(Max))?;

        let initial = self.value("Initial Physical Damage", Key::Q)?;
        let initial = self.map_changed(Key::Q, "Initial base damage", initial)?;
        let bleed = self.damage(Q(_1))?;

        let minimum = self.value("Minimum Bonus Physical Damage", Key::Q)?;
        let minimum = self.map_changed(Key::Q, "Minimum bonus damage", minimum)?;
        let minimum = self.map_changed(Key::Q, "Minimum bonus damage bonus AD ratio", minimum)?;

        let maximum = self.value("Maximum Bonus Physical Damage", Key::Q)?;
        let maximum = self.map_changed(Key::Q, "Maximum bonus damage", maximum)?;
        let maximum = self.map_changed(Key::Q, "Maximum bonus damage bonus AD ratio", maximum)?;

        let dash = self.damage(E(Min))?;
        let flurry = self.value("Flurry Physical Damage", Key::E)?;
        let flurry = self.map_changed(Key::E, "Flurry base damage", flurry)?;
        let flurry = self.map_changed(Key::E, "Flurry AD ratio", flurry)?;

        self.set(Q(Min), f![initial + bleed + minimum])
            .set(Q(Max), f![initial + bleed + maximum])
            .set(E(Max), f![dash + flurry])
            .take("Physical Damage", R(Void))?
            .take("Physical Damage per Packmate", R(_1))?
            .combos([
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
