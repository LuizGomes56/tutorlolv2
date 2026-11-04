use super::*;

impl Generator for Graves {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Physical Damage", Min),
                ("Total Physical Damage", Max),
            ],
        )
        .ability(Key::W, [("Magic Damage", Void)])
        .ability(
            Key::R,
            [
                ("Physical Damage", Void),
                ("Reduced Damage", _1),
            ],
        )
        .combos([
            &[
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(R(Void)),
                Attack,
                Attack,
                Attack,
            ],
            &[Ability(Q(Max)), Attack, Ability(R(Void))],
            &[Ability(W(Void)), Ability(Q(Max)), Ability(R(Void))],
            &[
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(R(Void)),
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[Ability(W(Void)), Attack, Ability(Q(Max)), Attack, Attack],
            &[Attack, Attack, Ability(Q(Max)), Attack],
            &[Ability(W(Void)), Attack, Attack, Ability(Q(Max)), Attack],
        ])?
        .end()
    }
}
