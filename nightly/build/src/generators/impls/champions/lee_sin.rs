use super::*;

impl Generator for LeeSin {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(1, Key::Q, [("Physical Damage", _1)])
            .ability(
                Key::Q,
                [
                    ("Maximum Physical Damage", Max),
                    ("Minimum Physical Damage", Min),
                ],
            )
            .ability_nth(1, Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Collision Physical Damage", Max),
                    ("Physical Damage", Min),
                ],
            )
            .combos([
                &[Ability(Q(Min)), Ability(Q(Min)), Ability(R(Max))],
                &[
                    Ability(Q(Min)),
                    Ability(R(Max)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(R(Max)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Min)), Ability(R(Max)), Ability(Q(Min)), Attack],
                &[
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Ability(Q(Max)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
            ])?
            .end()
    }
}
