use super::*;

impl Generator for Vladimir {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Increased Damage", Max),
                ("Magic Damage", Min),
            ],
        )
        .ability(
            Key::W,
            [
                ("Magic Damage Per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(
            Key::E,
            [
                ("Maximum Magic Damage", Max),
                ("Minimum Magic Damage", Min),
            ],
        )
        .ability(Key::R, [("Magic damage", Void)])
        .combos([
            &[
                Ability(E(Max)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Max)),
                Ability(W(Max)),
            ],
            &[
                Ability(Q(Max)),
                Ability(R(Void)),
                Ability(E(Max)),
                Ability(W(Max)),
            ],
            &[Ability(E(Max)), Ability(W(Max)), Ability(Q(Max))],
            &[Ability(E(Max)), Ability(Q(Max)), Ability(W(Max))],
            &[Ability(E(Max)), Ability(Q(Max))],
            &[
                Ability(E(Max)),
                Ability(R(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(W(Max)),
            ],
        ])?
        .end()
    }
}
