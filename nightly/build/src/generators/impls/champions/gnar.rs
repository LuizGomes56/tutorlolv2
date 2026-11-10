use super::*;

impl Generator for Gnar {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(
            1,
            Key::Q,
            [
                ("Physical Damage", Max),
                ("Reduced Damage", Min),
            ],
        )
        .ability(Key::Q, [("Physical Damage", Mega)])
        .ability_nth(1, Key::W, [("Bonus Magic Damage", Void)])
        .ability(Key::W, [("Physical Damage", Mega)])
        .ability_nth(1, Key::E, [("Physical Damage", Void)])
        .ability(Key::E, [("Physical Damage", Mega)])
        .ability(
            Key::R,
            [
                ("Increased Damage", Max),
                ("Physical Damage", Min),
            ],
        )
        .combos([
            &[Ability(E(Void)), Ability(R(Max))],
            &[
                Ability(E(Void)),
                Attack,
                Ability(R(Max)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(R(Max)),
                Attack,
                Ability(Q(Max)),
                Attack,
            ],
            &[
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
            ],
            &[Attack, Ability(E(Void)), Ability(Q(Max)), Attack],
        ])?
        .end()
    }
}
