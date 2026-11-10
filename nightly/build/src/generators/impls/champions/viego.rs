use super::*;

impl Generator for Viego {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Bonus Physical Damage", Void),
                ("Minimum Bonus Damage", Min),
                ("Physical Damage", Max),
            ],
        )
        .ability(Key::W, [("Magic Damage", Void)])
        .ability(Key::R, [("Physical Damage", Void)])
        .combos([
            &[
                Ability(W(Void)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Void)),
                Attack,
            ],
            &[Ability(W(Void)), Attack, Ability(Q(Void)), Attack],
            &[Ability(Q(Void)), Attack, Ability(W(Void)), Attack],
            &[Ability(W(Void)), Attack, Ability(Q(Void))],
            &[
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[Ability(W(Void)), Ability(Q(Void))],
        ])?
        .end()
    }
}
