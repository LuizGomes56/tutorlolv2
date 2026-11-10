use super::*;

impl Generator for Garen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Bonus Physical Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Physical Damage Per Spin", Min),
                    ("Critical Strike Damage Per Spin", _1),
                ],
            )
            .clone_with(E(Min), E(Max), |e| f![7 * (e)])?
            .comment(E(Max), "Total damage of 7 spins")?
            .ability(Key::R, [("True Damage", Void)])
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
