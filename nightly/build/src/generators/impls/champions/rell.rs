use super::*;

impl Generator for Rell {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability_nth(1, Key::W, [("Magic Damage", Void)])
            .ability(Key::W, [("Bonus Magic Damage", _1)])
            .ability(Key::E, [("Bonus Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Magic Damage Per Tick", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .end()
    }
}
