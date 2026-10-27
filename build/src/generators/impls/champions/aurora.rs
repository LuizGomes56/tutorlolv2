use super::*;

impl Generator for Aurora {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Innate */])
            .modify(P(Void), |_| {
                f![(0.01 + 0.00027 * AbilityPower) * EnemyMaxHealth]
            })?
            .comment(P(Void), "Third stack bonus magic damage")?
            .ability(
                Key::Q,
                [
                    (1, Max),   /* Maximum Magic Damage */
                    (2, Min),   /* Minimum Magic Damage */
                    (3, _1Max), /* Subsequent Bolt Maximum Magic Damage */
                    (4, _1Min), /* Subsequent Bolt Minimum Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combo([
                Ability(R(Void)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
            ])?
            .combo([
                Ability(R(Void)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
            ])?
            .combo([Ability(Q(Min)), Ability(E(Void)), Ability(Q(Min))])?
            .combo([Ability(Q(Min)), Attack, Ability(E(Void)), Ability(Q(Min))])?
            .end()
    }
}
