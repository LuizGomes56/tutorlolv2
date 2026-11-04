use super::*;

impl Generator for Sivir {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Minimum Damage", Min),
                ("Physical Damage", Void),
                ("Total Maximum Champion Damage", Max),
            ],
        )
        .ability(
            Key::W,
            [
                ("Bounce Critical Damage", Max),
                ("Bounce Damage", Min),
            ],
        )
        .combos([
            &[Ability(Q(Void)), Attack, Ability(W(Max)), Attack, Attack],
            &[Attack, Ability(W(Max)), Ability(Q(Void)), Attack, Attack],
            &[
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
            ],
            &[Attack, Ability(W(Max)), Ability(Q(Void))],
            &[Attack, Ability(Q(Void)), Attack],
        ])?
        .end()
    }
}
