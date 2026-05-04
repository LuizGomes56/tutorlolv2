use super::*;

impl Generator for Yunara {
    fn generate(&mut self) -> MayFail {
        self.ability(
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
                (0, _1), /* Initial Magic Damage */
                (1, _2), /* Linger Magic Damage per Tick */
                (2, _3), /* Total Expanded Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Arc of Ruin Base Damage */])
        .end()
    }
}
