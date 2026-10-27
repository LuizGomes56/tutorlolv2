use super::*;

impl Generator for Taliyah {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (3, Min), /* Magic Damage */
                (7, Max), /* Total Magic Damage */
                (2, _1),  /* Empowered Damage */
                (4, _2),  /* Reduced Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (1, Void), /* Magic Damage */
                (0, _1),   /* Detonation Magic Damage */
                (2, _2),   /* Total Maximum Detonation Damage */
            ],
        )
        .end()
    }
}
