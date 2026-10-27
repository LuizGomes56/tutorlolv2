use super::*;

impl Generator for Lillia {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Innate */])
            .modify(P(Void), |_| {
                f![(0.05 + 0.000125 * AbilityPower) * EnemyMaxHealth]
            })?
            .comment(P(Void), "Dream Dust total magic damage over 3 seconds")?
            .ability(
                Key::Q,
                [
                    (1, Min), /* Magic Damage */
                    (3, Max), /* Total Mixed Damage */
                ],
            )
            .damage_type(Q(Min), Magic)?
            .ability(
                Key::W,
                [
                    (2, Min), /* Magic Damage */
                    (0, Max), /* Increased Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combo([
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(R(Void)),
                Ability(W(Max)),
                Ability(Q(Min)),
            ])?
            .combo([Ability(Q(Max)), Ability(R(Void))])?
            .combo([
                Ability(E(Void)),
                Ability(R(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(W(Max)),
            ])?
            .combo([
                Ability(E(Void)),
                Ability(R(Void)),
                Ability(W(Max)),
                Ability(Q(Max)),
            ])?
            .combo([
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(Q(Min)),
            ])?
            .combo([Ability(E(Void)), Ability(W(Max)), Ability(Q(Max))])?
            .end()
    }
}
