use super::*;

impl Generator for Rumble {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Overheated */])
            .modify(P(Void), |_| {
                f![5 + 39.12 / 17 * (Level - 1) + 0.25 * AbilityPower + 0.04 * EnemyMaxHealth]
            })?
            .comment(P(Void), "Overheated bonus magic damage on-hit")?
            .ability(
                Key::Q,
                [
                    (12, Min),   /* Minimum Magic Damage */
                    (8, Max),    /* Maximum Magic Damage */
                    (10, _1Min), /* Minimum Enhanced Damage */
                    (6, _1Max),  /* Maximum Enhanced Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (3, Min),   /* Magic Damage */
                    (10, Max),  /* Total Magic Damage */
                    (0, _1Min), /* Enhanced Damage */
                    (6, _1Max), /* Total Enhanced Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Min), /* Magic Damage per Second */
                    (2, Max), /* Maximum Magic Damage */
                ],
            )
            .combo([
                Ability(R(Max)),
                Ability(E(Min)),
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
                Attack,
            ])?
            .combo([
                Ability(R(Max)),
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
                Attack,
                Ability(E(Min)),
                Attack,
            ])?
            .combo([Ability(E(Max)), Ability(Q(Max)), Attack, Attack, Attack])?
            .combo([
                Ability(Q(Max)),
                Ability(R(Max)),
                Attack,
                Ability(E(Min)),
                Attack,
                Ability(E(Min)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
                Attack,
            ])?
            .combo([
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
                Attack,
                Ability(E(Min)),
                Attack,
            ])?
            .end()
    }
}
