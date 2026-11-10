use super::*;

impl Generator for Draven {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Bonus Physical Damage", Void)])
            .ability(Key::E, [("Physical Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Minimum Physical Damage", _1Min),
                    ("Minimum Total Damage", Min),
                    ("Physical Damage", _1Max),
                    ("Total Physical Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack, Attack, Attack],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Attack, Attack, Attack],
            ])?
            .end()
    }
}
