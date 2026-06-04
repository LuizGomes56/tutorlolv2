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
        .modify(P(_1), |dmg| 15.plus(dmg))?
        .ability(Key::Q, [(0, Void) /* Physical Damage */])
        .ability(Key::W, [(1, Void) /* Magic Damage */])
        .ability(Key::R, [(1, Min) /* Physical Damage Per Shot */])
        .clone_with(R(Min), R(Max), |dmg| {
            let shots = 2.2222.times(CritChance).div(100);
            /* Maybe include IE bonus shots in the future */
            dmg.parenthesize().times(shots)
        })?
        .comment(R(Max), "Maximum Damage without IE")?
        .end()
    }
}
