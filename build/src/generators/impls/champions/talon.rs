use super::*;

impl Generator for Talon {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Description 1", Void)])
            .ability(
                Key::Q,
                [
                    ("Critical Physical Damage", Max),
                    ("Physical Damage", Min),
                ],
            )
            .ability(
                Key::W,
                [
                    ("Physical Damage", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Physical Damage", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .combos([
                &[Ability(W(Max)), Ability(Q(Max)), Ability(R(Max)), Attack],
                &[
                    Ability(W(Max)),
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Max)), Ability(R(Max)), Ability(Q(Max)), Attack],
                &[Ability(W(Max)), Attack, Ability(Q(Max)), Attack],
                &[Ability(W(Max)), Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
