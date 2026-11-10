use super::*;

impl Generator for Nasus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Bonus Physical Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Magic Damage", Min),
                    ("Magic Damage Per Tick", _1),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Magic Damage Per Tick", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(E(Min)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[Ability(E(Min)), Ability(Q(Void)), Ability(E(Min)), Attack],
                &[Ability(E(Max)), Attack, Ability(Q(Void))],
                &[Attack, Ability(Q(Void)), Ability(E(Max)), Attack],
            ])?
            .end()
    }
}
