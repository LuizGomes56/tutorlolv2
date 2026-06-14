use super::*;

impl Generator for Twitch {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Min) /* Deadly Venom */])
            .clone_with(P(Min), P(Max), |dmg| dmg.times(6))?
            .clone_with(P(Max), P(_1Min), |dmg| dmg.times(6))?
            .clone_with(P(_1Min), P(_1Max), |dmg| dmg.times(6))?
            .ability(
                Key::E,
                [
                    (0, Void), /* Base Physical Damage */
                    (1, Max),  /* Maximum Mixed Damage */
                    (2, Min),  /* Minimum Mixed Damage */
                    (3, _1),   /* Physical Damage Per Stack */
                ],
            )
            .end()
    }
}
