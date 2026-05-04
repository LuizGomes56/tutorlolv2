use super::*;

impl Generator for Gwen {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Center Damage per Snip */
                (1, _2), /* Damage per Snip */
                (2, _3), /* Final Snip Center Damage */
                (3, _4), /* Final Snip Damage */
                (4, _5), /* Maximum Center Damage */
                (5, _6), /* Maximum Damage */
                (6, _7), /* Minimum Center Damage */
                (7, _8), /* Minimum Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Damage with  A Thousand Cuts */
                (1, _2), /* Magic Damage per Needle */
                (2, _3), /* Maximum Total Damage */
                (3, _4), /* Second Cast Total Damage */
                (6, _5), /* Third Cast Total Damage */
            ],
        )
        .end()
    }
}
