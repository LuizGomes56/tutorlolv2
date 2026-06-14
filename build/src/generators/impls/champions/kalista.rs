use super::*;

impl Generator for Kalista {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Bonus Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1),   /* Bonus Damage per Additional Stack */
                    (2, Void), /* Physical Damage */
                ],
            );

        let e_dmg = self.merge_damage([E(_1), E(Void)], |[e1, e]| {
            e.parenthesize().plus(e1.times(Stacks).parenthesize())
        })?;

        self.set_damage(E(Void), e_dmg)?.end()
    }
}
