use super::*;

impl Generator for Jhin {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Damage per Target Death */
                (1, _2), /* Maximum Final Bounce Physical Damage */
                (2, _3), /* Physical Damage */
            ],
        )
        .ability(Key::W, [(1, _1) /* Physical Damage */])
        .ability_nth(1, Key::W, [(1, _2) /* Physical Damage */])
        .ability(
            Key::E,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Reduced Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Maximum Fourth Shot Damage */
                (1, _2), /* Maximum Physical Damage per Bullet */
                (2, _3), /* Minimum Fourth Shot Damage */
                (3, _4), /* Minimum Physical Damage per Bullet */
            ],
        )
        .ability_nth(
            1,
            Key::R,
            [
                (0, _5), /* Maximum Fourth Shot Damage */
                (1, _6), /* Maximum Physical Damage per Bullet */
                (2, _7), /* Minimum Fourth Shot Damage */
                (3, _8), /* Minimum Physical Damage per Bullet */
            ],
        )
        .ability_nth(
            2,
            Key::R,
            [
                (0, _1Min), /* Maximum Fourth Shot Damage */
                (1, _2Min), /* Maximum Physical Damage per Bullet */
                (2, _3Min), /* Minimum Fourth Shot Damage */
                (3, _4Min), /* Minimum Physical Damage per Bullet */
            ],
        )
        .end()
    }
}
