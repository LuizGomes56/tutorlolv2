use super::*;

impl Generator for Leblanc {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Total Magic Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .ability_nth(1, Key::W, [(0, _2) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, _1), /* Fracture Magic Damage */
                (1, _2), /* Magic Damage */
                (2, _3), /* Total Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Application Magic Damage */
                (1, _2), /* Fracture Magic Damage */
                (2, _3), /* Magic Damage */
                (3, _4), /* Mark Magic Damage */
                (4, _5), /* Orb Magic Damage */
                (5, _6), /* Total Magic Damage */
            ],
        )
        .ability_nth(
            1,
            Key::R,
            [
                (0, _7),    /* Application Magic Damage */
                (1, _8),    /* Fracture Magic Damage */
                (2, _1Min), /* Magic Damage */
                (3, _2Min), /* Mark Magic Damage */
                (4, _3Min), /* Orb Magic Damage */
                (5, _4Min), /* Total Magic Damage */
            ],
        )
        .ability_nth(
            2,
            Key::R,
            [
                (0, _5Min), /* Application Magic Damage */
                (1, _6Min), /* Fracture Magic Damage */
                (2, _7Min), /* Magic Damage */
                (3, _8Min), /* Mark Magic Damage */
                (4, _1Max), /* Orb Magic Damage */
                (5, _2Max), /* Total Magic Damage */
            ],
        )
        .ability_nth(
            3,
            Key::R,
            [
                (0, _3Max), /* Application Magic Damage */
                (1, _4Max), /* Fracture Magic Damage */
                (2, _5Max), /* Magic Damage */
                (3, _6Max), /* Mark Magic Damage */
                (4, _7Max), /* Orb Magic Damage */
                (5, _8Max), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
