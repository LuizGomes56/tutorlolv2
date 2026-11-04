use super::*;

impl Generator for Karma {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(1, Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::Q,
                [
                    ("Total Magic Damage", _1),
                    ("Total Damage", _2),
                ],
            )
            .comment(Q(_1), "Mantra empowered impact")?
            .comment(Q(_2), "Mantra empowered impact and Soulflare")?
            .ability_nth(
                1,
                Key::W,
                [
                    ("Magic Damage", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .combos([
                &[Ability(Q(Void)), Ability(W(Max)), Attack, Attack, Attack],
                &[
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(W(Max)), Attack, Ability(Q(Void)), Attack],
                &[Ability(W(Max)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(W(Max)), Attack, Attack],
                &[Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
