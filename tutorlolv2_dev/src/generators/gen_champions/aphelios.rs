use super::*;

impl Generator for Aphelios {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Bonus Attack Damage */])
            .end()
    }
}
