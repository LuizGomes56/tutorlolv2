use super::*;

impl Generator for Akshan {
    fn generate(&mut self) -> MayFail {
        // 15 / 40 / 80 / 150 at levels 1 / 6 / 11 / 16
        let dirty_fighting = stringify!(match ctx.level {
            ..6 => 15,
            6..11 => 40,
            11..16 => 80,
            16.. => 150,
        });

        self.ability(Key::P, [(1, Void) /* Innate */])
            .modify(P(Void), |_| f![(dirty_fighting) + 0.6 * AbilityPower])?
            .damage_type(P(Void), Magic)?
            .comment(P(Void), "Dirty Fighting third stack bonus magic damage")?
            .ability(
                Key::Q,
                [
                    (1, Min), /* Physical Damage */
                    (2, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Physical Damage per Shot */])
            .ability(
                Key::R,
                [
                    (1, _1Max), /* Damage to target on 67% missing hp */
                    (3, Max),   /* Maximum Physical Damage per Bullet */
                    (4, _1Min), /* Minimum Charged Physical Damage */
                    (5, Min),   /* Minimum Physical Damage per Bullet */
                ],
            )
            .combos([
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[Attack, Attack, Ability(Q(Max))],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
