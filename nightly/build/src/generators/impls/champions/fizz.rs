use super::*;

impl Generator for Fizz {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .damage_type(Q(Void), Magic)?
            .ability(
                Key::W,
                [
                    ("Active Magic Damage", Void),
                    ("Active On-Hit Magic Damage", _1),
                    ("Total Passive Magic Damage", _2),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Guppy Damage", Min),
                    ("Chomper Damage", _1),
                    ("Gigalodon Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Ability(W(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(R(Max))],
                &[Ability(E(Void)), Ability(W(Void)), Ability(Q(Void))],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
