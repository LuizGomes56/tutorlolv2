use super::*;

impl Generator for Brand {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, Void), /* Ablaze */
                (3, _1),   /* Description 2 */
            ],
        )
        .modify(P(_1), |p| p.parenthesize().times(EnemyMaxHealth))?
        .ability(Key::Q, [(0, Void) /* Magic Damage */])
        .ability(
            Key::W,
            [
                (0, Max), /* Increased Damage */
                (1, Min), /* Magic Damage */
            ],
        )
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, Min), /* Magic Damage */
                (2, Max), /* Total Single-Target Damage */
            ],
        )
        .end()
    }
}
