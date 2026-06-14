use super::*;

impl Generator for Yorick {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        /* Missing Ghoul specifications */
        self.ability(Key::P, [(2, Void) /* Innate - Final Service */])
            .ability(Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(Key::E, [(3, Void) /* Magic Damage */])
            .end()
    }
}
