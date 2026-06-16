use super::*;

impl Generator for Orianna {
    fn generate(&mut self) -> MayFail {
        let pmax = self
            .ability(Key::P, [(1, Min) /* Innate */])
            .merge_damage([P(Min)], |[p]| f![1.4 * p])?;

        self.clone_to(P(Min), P(Max), pmax)?
            .ability(
                Key::Q,
                [
                    (0, Max), /* Magic Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
