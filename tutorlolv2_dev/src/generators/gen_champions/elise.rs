use super::*;

impl Generator for Elise {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1), /* Innate - Spider Form */
                (3, _2), /* Innate - Spider Form [1] */
            ],
        )
        .ability(Key::Q, [(1, _1) /* Magic Damage */])
        .ability_nth(1, Key::Q, [(1, _2) /* Magic Damage */])
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .end()
    }
}
