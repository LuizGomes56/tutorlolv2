use super::*;

impl Generator for Fiddlesticks {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (3, Min), /* Magic Damage */
                (1, Max), /* Increased Magic Damage */
            ],
        )
        .modify(Q(Min), |_| {
            f![(0.04 + 0.005 * (QLevel - 1) + 0.0003 * AbilityPower) * EnemyCurrentHealth]
        })?
        .modify(Q(Max), |_| {
            f![(0.08 + 0.01 * (QLevel - 1) + 0.0006 * AbilityPower) * EnemyCurrentHealth]
        })?
        .comment(Q(Max), "Against feared or recently feared enemies")?
        .ability(
            Key::W,
            [
                (1, Min), /* Damage per Instance */
                (7, Max), /* Total Magic Damage */
                (3, _1),  /* Last Tick of Damage */
            ],
        )
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, Min), /* Magic Damage per Tick */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .combo([
            Ability(R(Max)),
            Ability(Q(Max)),
            Ability(E(Void)),
            Ability(W(Max)),
        ])?
        .combo([Ability(Q(Max)), Ability(W(Max)), Ability(E(Void)), Attack])?
        .combo([
            Ability(E(Void)),
            Ability(R(Max)),
            Ability(Q(Max)),
            Ability(W(Max)),
        ])?
        .combo([Ability(Q(Max)), Ability(E(Void)), Ability(W(Max))])?
        .combo([Ability(E(Void)), Ability(Q(Max)), Ability(W(Max))])?
        .combo([
            Ability(R(Max)),
            Ability(Q(Max)),
            Ability(W(Max)),
            Ability(E(Void)),
            Attack,
        ])?
        .end()
    }
}
