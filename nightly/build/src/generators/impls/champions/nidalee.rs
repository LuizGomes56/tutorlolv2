use super::*;

impl Generator for Nidalee {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(
            1,
            Key::Q,
            [
                ("Minimum Magic Damage", Min),
                ("Maximum Magic Damage", Max),
            ],
        )
        .ability(
            Key::Q,
            [
                ("Minimum Magic Damage", _1Min),
                ("Maximum Magic Damage", _1Max),
            ],
        )
        .ability_nth(
            1,
            Key::W,
            [
                ("Total Magic Damage", Void),
                ("Magic Damage Per Tick", _1),
            ],
        )
        .ability(Key::W, [("Magic Damage", _2)])
        .ability(Key::E, [("Magic Damage", Void)])
        .combos([
            &[
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Attack,
            ],
            &[Attack, Ability(E(Void)), Ability(Q(Max)), Ability(W(Void))],
            &[Ability(W(Void)), Ability(Q(Max)), Attack],
            &[Ability(E(Void)), Ability(W(Void)), Ability(Q(Max)), Attack],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Min)),
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
