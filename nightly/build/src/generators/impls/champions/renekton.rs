use super::*;

impl Generator for Renekton {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Enhanced Damage", Max),
                ("Physical Damage", Min),
            ],
        )
        .ability(
            Key::W,
            [
                ("Physical Damage Per Hit", Min),
                ("Total Physical Damage", Max),
            ],
        )
        .ability(
            Key::E,
            [
                ("Enhanced Physical Damage", _1Min),
                ("Physical Damage", Min),
                ("Total Enhanced Damage", _1Max),
                ("Total Physical Damage", Max),
            ],
        )
        .ability(
            Key::R,
            [
                ("Magic Damage Per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .combos([
            &[
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Ability(R(Max)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
            ],
            &[
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Ability(R(Max)),
                Ability(E(Min)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(E(Min)),
            ],
            &[
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(Q(Max)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Attack,
            ],
            &[
                Ability(E(Min)),
                Attack,
                Ability(W(Max)),
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
            ],
            &[
                Ability(E(Min)),
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
            ],
            &[
                Ability(E(Min)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
                Attack,
            ],
            &[Attack, Ability(W(Max)), Attack],
            &[Ability(E(Min)), Attack, Ability(Q(Max)), Ability(E(Min))],
        ])?
        .end()
    }
}
