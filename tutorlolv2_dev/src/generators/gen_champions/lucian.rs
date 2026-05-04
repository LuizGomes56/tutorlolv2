use super::*;

impl Generator for Lucian {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(Key::W, [(1, _1) /* Magic Damage */])
            .ability_nth(1, Key::W, [(1, _2) /* Magic Damage */])
            .ability_nth(2, Key::W, [(1, _3) /* Magic Damage */])
            .ability(Key::R, [(1, _1) /* Physical Damage Per Shot */])
            .end()
    }
}
