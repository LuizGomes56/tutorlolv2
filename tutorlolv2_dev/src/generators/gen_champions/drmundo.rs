use super::*;

impl Generator for DrMundo {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Max), /* Magic Damage */
                (2, Min), /* Minimum Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, Min), /* Magic Damage */
                (1, _1),  /* Magic Damage per Tick */
                (2, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (1, Max), /* Maximum Bonus Physical Damage */
                (4, Min), /* Minimum Bonus Physical Damage */
            ],
        )
        .end()
    }
}
