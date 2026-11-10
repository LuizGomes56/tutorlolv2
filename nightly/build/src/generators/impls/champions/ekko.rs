use super::*;

impl Generator for Ekko {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate - Resonance", Void)])
            .ability(
                Key::Q,
                [
                    ("Initial Magic Damage", _1Min),
                    ("Return Magic Damage", _1Max),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(Key::E, [("Bonus Magic Damage", Void)])
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Max)), Ability(E(Void)), Ability(E(Void)), Attack],
            ])?
            .end()
    }
}
