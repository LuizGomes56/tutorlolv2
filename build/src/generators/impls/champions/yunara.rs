use super::*;

impl Generator for Yunara {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Passive Bonus Magic Damage", Void)])
            .damage_type(Q(Void), Magic)?
            .ability_nth(
                1,
                Key::W,
                [
                    ("Initial Magic Damage", Void),
                    ("Linger Magic Damage per Tick", _1),
                    ("Total Expanded Damage", _2),
                ],
            )
            .ability(Key::R, [("Arc of Ruin Base Damage", Void)])
            .damage_type(R(Void), Magic)?
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
