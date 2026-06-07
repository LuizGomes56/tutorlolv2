use super::*;

impl Generator for Zyra {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        /* Missing plants specifications */
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
