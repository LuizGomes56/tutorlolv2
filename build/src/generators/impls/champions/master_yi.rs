use super::*;

impl Generator for MasterYi {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Ability {
                name: "Double Strike".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Second strike".into(),
                damage: f![0.5 * AttackDamage],
            },
        )
        .ability(
            Key::Q,
            [
                (7, Min), /* Primary Physical Damage */
                (5, Max), /* Maximum Single-Target Damage */
                (8, _1),  /* Reduced Damage per hit */
            ],
        )
        .ability(Key::E, [(0, Void) /* Bonus True Damage */])
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
