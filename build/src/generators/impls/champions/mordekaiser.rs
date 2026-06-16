use super::*;

impl Generator for Mordekaiser {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        let ap_scaling = &self.scaling(Key::P, 0)?[0].render(Level)?;

        self.ability(Key::P, [(1, Void) /* Darkness Rise [1] */])
            .modify(P(Void), |r| f![5 + ap_scaling + r])?
            .ability(
                Key::Q,
                [
                    (0, Max), /* Isolated Damage Increase */
                    (1, Min), /* Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .end()
    }
}
