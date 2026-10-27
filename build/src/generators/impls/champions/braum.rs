use super::*;

impl Generator for Braum {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, Void), /* Description 1 */
                (1, _1),   /* Description 2 */
            ],
        )
        .ability(Key::Q, [(0, Void) /* Magic Damage */])
        .ability(Key::R, [(0, Void) /* Magic Damage */]);

        // Bonus magic damage equal to 40% of the Concussive Blows damage
        let concussive_blows = self.damage_of(P(Void))?.clone();

        self.modify(P(_1), |_| f![0.4 * (concussive_blows)])?
            .combos([
                &[Ability(Q(Void)), Ability(R(Void))],
                &[Ability(Q(Void)), Attack, Attack, Attack],
                &[Ability(R(Void)), Ability(Q(Void)), Attack, Attack, Attack],
                &[Ability(Q(Void)), Attack, Ability(R(Void)), Attack, Attack],
                &[Attack, Ability(Q(Void)), Attack, Attack],
            ])?
            .end()
    }
}
