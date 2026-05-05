use super::*;

impl Generator for Volibear {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Innate */
                (1, _2), /* Lightning Claws */
            ],
        )
        .ability(Key::Q, [(1, _1) /* Bonus Physical Damage */])
        .ability(Key::W, [(2, _1) /* Physical Damage */])
        .ability(
            Key::E,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Non-Champion Capped Damage */
            ],
        )
        .ability(Key::R, [(1, _1) /* Physical Damage */])
        .end()
    }
}
