use super::*;

impl Generator for Quinn {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* Physical damage */])
            .ability_nth(1, Key::R, [(0, Void) /* Physical Damage */])
            .end()
    }
}
