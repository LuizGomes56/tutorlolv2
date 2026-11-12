use super::*;

impl GeneratorV2 for MasterYi {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Damage {
                name: "Double Strike".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Second strike".into(),
                formula: f![0.5 * AttackDamage],
            },
        )?
        .take("Primary Physical Damage", Q(Min))?
        .take("Maximum Single-Target Damage", Q(Max))?
        .take("Reduced Damage per hit", Q(_1))?
        .take("Bonus True Damage", E(Void))?
        .combos([
            &[
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
            ],
            &[
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Attack,
            ],
            &[
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
