use super::*;

impl Generator for Diana {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Innate - Moonsilver Blade */
                (2, _2), /* Innate - Moonsilver Blade [1] */
                (3, _3), /* Innate - Moonsilver Blade [2] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Magic Damage */])
        .ability(
            Key::W,
            [
                (0, _1), /* Magic Damage per Orb */
                (3, _2), /* Total Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Bonus Damage Per Champion */
                (1, _2), /* Magic Damage */
                (3, _3), /* Total Damage Vs. 5 Champions */
            ],
        )
        .end()
    }
}
