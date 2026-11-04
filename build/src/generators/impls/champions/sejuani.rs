use super::*;

impl Generator for Sejuani {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate - Icebreaker", Void)])
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Physical Damage", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Increased Damage", Max),
                    ("Magic Damage", Min),
                ],
            )
            .combos([
                &[
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(R(Max)), Ability(W(Max)), Attack],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                ],
                &[Ability(R(Max)), Ability(W(Max)), Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
