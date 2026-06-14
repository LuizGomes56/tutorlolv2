use super::*;

impl Generator for Akali {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1), /* Swinging Kama */
                (3, _2), /* Swinging Kama [1] */
            ],
        )
        .merge_sum([P(_1), P(_2)], P(Void))?
        .ability(Key::Q, [(0, Void) /* Magic Damage */])
        /* Missing first cast damage */
        .ability(
            Key::E,
            [
                (0, Min), /* Magic Damage */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1),  /* Magic Damage */
                (1, Max), /* Maximum Magic Damage */
                (2, Min), /* Minimum Magic Damage */
            ],
        )
        .end()
    }
}
