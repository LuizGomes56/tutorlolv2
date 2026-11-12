use super::*;

impl GeneratorV2 for Naafiri {
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
        .take("Minimum Total Physical Damage", Q(Min))?
        .take("Maximum Total Physical Damage", Q(Max))?
        .take("Total Bleed Physical Damage", Q(_1))?
        .take("Dash Physical Damage", E(Min))?
        .take("Total Physical Damage", E(Max))?
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
