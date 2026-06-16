use super::*;

impl Generator for Lucian {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, Void), /* Innate */
                (2, _1),   /* Innate - Vigilance */
            ],
        )
        .modify(P(_1), |dmg| f![15 + dmg])?
        .ability(Key::Q, [(0, Void) /* Physical Damage */])
        .ability(Key::W, [(1, Void) /* Magic Damage */])
        .ability(Key::R, [(1, Min) /* Physical Damage Per Shot */])
        .clone_with(R(Min), R(Max), |dmg| {
            let shots = f![2.2222 * CritChance / 100];
            /* Maybe include IE bonus shots in the future */
            f![(dmg) * shots]
        })?
        .comment(R(Max), "Maximum Damage without IE")?
        .end()
    }
}
