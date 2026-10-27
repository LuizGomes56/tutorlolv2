use super::*;

impl Generator for Gwen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .modify(P(Void), |_| {
                f![(0.01 + 0.00006 * AbilityPower) * EnemyMaxHealth]
            })?
            .comment(P(Void), "A Thousand Cuts on-hit magic damage")?
            .ability(
                Key::Q,
                [
                    (6, Min), /* Minimum Center Damage */
                    (4, Max), /* Maximum Center Damage */
                    (0, _1),  /* Center Damage per Snip */
                    (2, _2),  /* Final Snip Center Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Min), /* Damage with A Thousand Cuts */
                    (2, Max), /* Maximum Total Damage */
                    (3, _1),  /* Second Cast Total Damage */
                    (6, _2),  /* Third Cast Total Damage */
                ],
            )
            .combo([
                Ability(R(Min)),
                Attack,
                Attack,
                Ability(R(Min)),
                Ability(Q(Max)),
                Attack,
                Ability(R(Min)),
            ])?
            .combo([Attack, Attack, Attack, Attack, Ability(Q(Max)), Attack])?
            .combo([
                Ability(R(Min)),
                Attack,
                Ability(R(Min)),
                Attack,
                Ability(R(Min)),
            ])?
            .combo([
                Ability(R(Min)),
                Attack,
                Attack,
                Ability(R(Min)),
                Attack,
                Attack,
                Ability(R(Min)),
                Ability(Q(Max)),
                Attack,
            ])?
            .combo([Attack, Attack, Ability(Q(Max)), Attack])?
            .end()
    }
}
