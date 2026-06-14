use super::*;

impl Generator for Yunara {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .ability(
                Key::Q,
                [
                    (0, _1), /* Active Bonus Magic Damage */
                    (3, _2), /* Combined Bonus Magic Damage */
                    (5, _3), /* Passive Bonus Magic Damage */
                ],
            )
            .ability(
                Key::W,
                [
                    (0, Min),  /* Initial Magic Damage */
                    (1, Void), /* Linger Magic Damage per Tick */
                    (2, Max),  /* Total Expanded Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Arc of Ruin Base Damage */])
            .end()
    }
}
