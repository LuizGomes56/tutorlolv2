use super::*;

impl Generator for Diana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate - Moonsilver Blade */])
            .ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Magic Damage per Orb */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1),  /* Bonus Damage Per Champion */
                    (1, Min), /* Magic Damage */
                    (3, Max), /* Total Damage Vs. 5 Champions */
                ],
            )
            .end()
    }
}
