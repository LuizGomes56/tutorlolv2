use super::*;

impl Generator for Ornn {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate - Temper [1]", Void)])
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Magic Damage Per Tick", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(Key::E, [("Physical Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Magic Damage", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[Ability(R(Min)), Ability(E(Void)), Ability(R(Min)), Attack],
                &[
                    Ability(R(Min)),
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack, Ability(E(Void)), Attack],
                &[Attack, Ability(W(Max)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(W(Max)), Attack],
            ])?
            .end()
    }
}
