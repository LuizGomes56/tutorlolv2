use super::*;

impl Generator for Morgana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Maximum Damage Per Tick", _1Max),
                    ("Maximum Total Damage", Max),
                    ("Minimum Damage Per Tick", _1Min),
                    ("Minimum Total Damage", Min),
                ],
            )
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
                    Ability(W(Min)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(W(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(W(Max))],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
