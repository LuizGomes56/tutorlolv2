use super::*;

impl Generator for Akshan {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Non-Champion Damage */
                (1, _2), /* Physical Damage */
                (2, _3), /* Total Physical Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Physical Damage per Shot */])
        .ability(
            Key::R,
            [
                (1, _1), /* Damage to target on 67% missing hp */
                (3, _2), /* Maximum Physical Damage per Bullet */
                (4, _3), /* Minimum Charged Physical Damage */
                (5, _4), /* Minimum Physical Damage per Bullet */
            ],
        )
        .end()
    }
}
