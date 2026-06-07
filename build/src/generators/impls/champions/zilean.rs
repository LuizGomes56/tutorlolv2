use super::*;

impl Generator for Zilean {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */]).end()
    }
}
