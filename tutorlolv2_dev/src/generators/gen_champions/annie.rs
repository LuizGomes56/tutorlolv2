use super::*;

impl Generator for Annie {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(Key::R, [(0, _1) /* Initial Magic Damage */])
            .ability_nth(1, Key::R, [(0, _2) /* Initial Magic Damage */])
            .end()
    }
}
