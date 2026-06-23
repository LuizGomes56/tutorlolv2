use super::*;

impl Generator for Varus {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max),   /* Maximum Physical Damage */
                (1, _1Max), /* Maximum Reduced Damage */
                (2, Min),   /* Minimum Physical Damage */
                (3, _1Min), /* Minimum Reduced Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, Max),   /* Active Maximum Magic Damage */
                (1, Min),   /* Active Minimum Magic Damage */
                (2, _1Min), /* Bonus Magic Damage */
                // (3, _2Min), /* Bonus Magic Damage at Max Stacks */
                // (4, _3), /* Bonus Magic Damage per Stack */
                (5, _1Max), /* Maximum Bonus Magic Damage at Max Stacks */
                            // (6, _2Max), /* Maximum Bonus Magic Damage per Stack */
            ],
        )
        .ability(Key::E, [(0, Void) /* Physical Damage */])
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .end()
    }
}
