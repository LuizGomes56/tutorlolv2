use super::*;

impl Generator for Braum {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, Void), /* Description 1 */
                (1, _1),   /* Description 2 */
            ],
        )
        .ability(Key::Q, [(0, Void) /* Magic Damage */])
        .ability(Key::R, [(0, Void) /* Magic Damage */]);

        let pmul = self
            .scaling(Key::P, 0)?
            .first()
            .map(|_| Some(0.0))
            // .map(Scaling::try_get_value)
            .flatten()
            .unwrap_or(0.4);

        self.modify(P(_1), |p| p.parenthesize().times(pmul))?.end()
    }
}
