use super::*;

impl Generator for Bard {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Magic Damage */]).end()
    }
}
