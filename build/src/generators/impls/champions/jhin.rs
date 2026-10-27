use super::*;

impl Generator for Jhin {
    fn generate(&mut self) -> MayFail {
        // 15% / 20% / 25% at levels 1 / 6 / 11
        let missing_health = stringify!(match ctx.level {
            ..6 => 0.15,
            6..11 => 0.2,
            11.. => 0.25,
        });

        self.ability(Key::P, [(0, Void) /* Description 1 [1] */])
            .modify(P(Void), |_| f![(missing_health) * EnemyMissingHealth])?
            .comment(P(Void), "Fourth shot bonus damage")?
            .ability(
                Key::Q,
                [
                    (2, Min), /* Physical Damage */
                    (1, Max), /* Maximum Final Bounce Physical Damage */
                ],
            )
            .ability(Key::W, [(1, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (3, Min),   /* Minimum Physical Damage per Bullet */
                    (1, Max),   /* Maximum Physical Damage per Bullet */
                    (2, _1Min), /* Minimum Fourth Shot Damage */
                    (0, _1Max), /* Maximum Fourth Shot Damage */
                ],
            )
            .combo([
                Attack,
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Ability(R(Max)),
                Ability(R(Max)),
                Ability(R(Max)),
                Ability(R(Max)),
                Ability(R(Max)),
            ])?
            .combo([
                Attack,
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Attack,
            ])?
            .combo([Ability(Q(Max)), Attack])?
            .combo([
                Attack,
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(Q(Max)),
                Attack,
            ])?
            .combo([Attack, Attack, Attack, Ability(Q(Max)), Attack])?
            .combo([
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
            ])?
            .end()
    }
}
