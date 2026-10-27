use super::*;

impl Generator for Garen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(
                Key::E,
                [
                    (5, Min), /* Physical Damage Per Spin */
                    (0, _1),  /* Critical Strike Damage Per Spin */
                ],
            )
            .clone_with(E(Min), E(Max), |e| f![7 * (e)])?
            .comment(E(Max), "Total damage of 7 spins")?
            .ability(Key::R, [(0, Void) /* True Damage */])
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(Q(Void)), Ability(E(Max)), Attack, Ability(R(Void))],
                &[Attack, Ability(Q(Void)), Ability(E(Max)), Attack],
                &[Ability(E(Max)), Attack, Ability(Q(Void)), Ability(R(Void))],
                &[Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
