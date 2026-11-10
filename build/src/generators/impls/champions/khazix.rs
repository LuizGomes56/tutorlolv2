use super::*;

impl Generator for Khazix {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability_nth(
                1,
                Key::Q,
                [
                    ("Isolated Target Physical Damage", Max),
                    ("Physical Damage", Min),
                ],
            )
            .ability_nth(1, Key::W, [("Physical Damage", Void)])
            .ability_nth(1, Key::E, [("Physical Damage", Void)])
            .damage_types([W(Void), E(Void)], Physical)?
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Void)),
                ],
                &[Attack, Ability(W(Void)), Ability(E(Void)), Ability(Q(Max))],
                &[Ability(E(Void)), Ability(Q(Max)), Ability(E(Void))],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(W(Void)), Ability(E(Void)), Ability(Q(Max)), Attack],
                &[Ability(E(Void)), Ability(Q(Max)), Attack, Ability(W(Void))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Max))],
                &[Ability(W(Void)), Ability(Q(Max)), Attack],
                &[Attack, Ability(W(Void)), Ability(Q(Max))],
            ])?
            .end()
    }
}
