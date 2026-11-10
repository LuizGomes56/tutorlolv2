use super::*;

impl Generator for Nunu {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Champion Magic Damage", Void)])
            .damage_type(Q(Void), Magic)?
            .ability(
                Key::W,
                [
                    ("Maximum Magic Damage", Max),
                    ("Minimum Magic Damage", Min),
                ],
            )
            .ability(
                Key::E,
                [
                    ("Magic Damage", _1),
                    ("Magic Damage Per Hit", Min),
                    ("Maximum Total Magic Damage", Max),
                    ("Total Magic Damage", _2),
                ],
            )
            .ability(Key::R, [("Maximum Magic Damage", Void)])
            .combos([
                &[
                    Ability(W(Min)),
                    Ability(W(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(E(Max)), Ability(R(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(E(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
