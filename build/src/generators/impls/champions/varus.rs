use super::*;

impl Generator for Varus {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Maximum Physical Damage", Max),
                ("Maximum Reduced Damage", _1Max),
                ("Minimum Physical Damage", Min),
                ("Minimum Reduced Damage", _1Min),
            ],
        )
        .ability(
            Key::W,
            [
                ("Active Maximum Magic Damage", Max),
                ("Active Minimum Magic Damage", Min),
                ("Bonus Magic Damage", _1Min),
                // ("Bonus Magic Damage at Max Stacks", _2Min),
                // ("Bonus Magic Damage per Stack", _3),
                ("Maximum Bonus Magic Damage at Max Stacks", _1Max),
                            // ("Maximum Bonus Magic Damage per Stack", _2Max),
            ],
        )
        .ability(Key::E, [("Physical Damage", Void)])
        .ability(Key::R, [("Magic Damage", Void)])
        .combos([
            &[
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(W(Max)),
                Ability(Q(Max)),
            ],
            &[
                Ability(R(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[Ability(E(Void)), Ability(R(Void)), Ability(Q(Max))],
            &[Attack, Attack, Attack, Ability(E(Void)), Attack],
            &[Attack, Ability(E(Void)), Attack, Attack, Ability(Q(Max))],
        ])?
        .end()
    }
}
