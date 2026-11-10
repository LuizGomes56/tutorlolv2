use super::*;

impl Generator for Ambessa {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Medarda Maxim", Void)])
            .damage_type(P(Void), Physical)?
            .ability_nth(
                1,
                Key::Q,
                [
                    ("Physical Damage", Min),
                    ("Increased Physical Damage", Max),
                ],
            )
            .ability(
                Key::Q,
                [
                    ("Physical Damage", _1Min),
                    ("Increased Physical Damage", _1Max),
                ],
            )
            .ability(
                Key::W,
                [
                    ("Physical Damage", Min),
                    ("Increased Physical Damage", Max),
                ],
            )
            .ability(
                Key::E,
                [
                    ("Physical Damage", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .ability(Key::R, [("Physical Damage", Void)])
            .combos([
                &[
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Max)),
                ],
            ])?
            .end()
    }
}
