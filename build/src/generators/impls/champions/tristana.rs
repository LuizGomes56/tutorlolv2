use super::*;

impl Generator for Tristana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    // (0, _1), /* Bonus Damage Per Stack */
                    // (1, _2), /* Full Stack Bonus Damage */
                    (2, Max),  /* Full Stack Physical Damage */
                    (3, Void), /* Magic Damage */
                    (4, Min),  /* Minimum Physical Damage */
                ],
            )
            .comment(E(Void), "Passive explosion damage on kill")?
            .ability(Key::R, [(1, Void) /* Magic Damage */])
            .end()
    }
}
