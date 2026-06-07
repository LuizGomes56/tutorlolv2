use super::*;

impl Generator for Blitzcrank {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            /* Missing E damage */
            .end()
    }
}
