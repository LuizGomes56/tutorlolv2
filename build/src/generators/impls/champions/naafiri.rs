use super::*;

impl Generator for Naafiri {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Ability {
                name: "Packmate".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Pet: Packmate basic attack".into(),
                damage: f![10 + 10 / 17 * (Level - 1) + 0.04 * BonusAd],
            },
        )
        .ability(
            Key::Q,
            [
                (6, Min), /* Minimum Total Physical Damage */
                (4, Max), /* Maximum Total Physical Damage */
                (7, _1),  /* Total Bleed Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Min), /* Dash Physical Damage */
                (2, Max), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, Void), /* Physical Damage */
                (1, _1),   /* Physical Damage per Packmate */
            ],
        )
        .combo([
            Ability(R(Void)),
            Attack,
            Ability(Q(Min)),
            Ability(E(Max)),
            Attack,
            Ability(Q(Min)),
            Attack,
        ])?
        .combo([
            Ability(Q(Min)),
            Ability(E(Max)),
            Attack,
            Ability(Q(Min)),
            Attack,
        ])?
        .combo([
            Ability(R(Void)),
            Attack,
            Ability(Q(Min)),
            Attack,
            Ability(E(Max)),
            Attack,
            Ability(Q(Min)),
            Attack,
        ])?
        .combo([
            Attack,
            Ability(Q(Min)),
            Ability(E(Max)),
            Attack,
            Ability(Q(Min)),
            Attack,
        ])?
        .combo([
            Ability(Q(Min)),
            Ability(Q(Min)),
            Attack,
            Ability(E(Max)),
            Attack,
        ])?
        .end()
    }
}
