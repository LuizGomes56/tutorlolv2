use super::*;

impl Generator for AurelionSol {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1),    /* Bonus Magic Damage */
                (1, _1Max), /* Magic Damage per Second */
                (2, _1Min), /* Magic Damage per Tick */
                (3, _2Max), /* Secondary Magic Damage per Second */
                (4, _2Min), /* Secondary Target Damage per Tick */
                (5, _3Max), /* Secondary Target Total Maximum Damage */
                (6, _4),    /* Total Maximum Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Min), /* Magic Damage per Tick */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .ability_nth(
            1,
            Key::R,
            [
                (0, _1Max), /* Empowered Magic Damage */
                (1, _1Min), /* Magic Damage */
            ],
        )
        .end()
    }
}
