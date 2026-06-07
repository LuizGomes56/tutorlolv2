use super::*;

impl Generator for Anivia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Magic Damage */
                (2, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Max), /* Enhanced Damage */
                (1, Min), /* Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, Max), /* Empowered Damage per Tick */
                (2, Min), /* Magic Damage per Tick */
            ],
        )
        .end()
    }
}
