use super::*;

impl Generator for Mel {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Innate */
                (1, _2), /* Innate - Searing Brilliance */
                (2, _3), /* Innate - Searing Brilliance [1] */
                (3, _4), /* Overwhelm */
                (4, _5), /* Overwhelm [1] */
                (5, _6), /* Overwhelm [2] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Initial Explosion Magic Damage */
                (1, _2), /* Magic Damage per Subsequent Explosion */
                (3, _3), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Replicated Projectile Magic Damage Modifier */
                (1, _2), /* Replicated Projectile Physical Damage Modifier */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Field Magic Damage per Second */
                (1, _2), /* Field Magic Damage per Tick */
                (3, _3), /* Orb Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Increased Stored Damage */
                (1, _2), /* Magic Damage */
                (2, _3), /* Stored Damage Increase per Stack */
            ],
        )
        .end()
    }
}
