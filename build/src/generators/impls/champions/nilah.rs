use super::*;

impl Generator for Nilah {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Maximum Physical Damage */
                (1, Min), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::E, [(0, Void) /* Physical Damage */])
        .ability(
            Key::R,
            [
                (0, _2),  /* Burst Physical Damage */
                (1, _1),  /* Maximum Total Physical Damage */
                (2, Min), /* Physical Damage per Tick */
                (3, Max), /* Total Physical Damage */
            ],
        )
        .end()
    }
}
