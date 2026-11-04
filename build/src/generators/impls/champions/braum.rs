use super::*;

impl Generator for Braum {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                ("Description 1", Void),
                ("Description 2", _1),
            ],
        )
        .ability(Key::Q, [("Magic Damage", Void)])
        .ability(Key::R, [("Magic Damage", Void)]);

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
