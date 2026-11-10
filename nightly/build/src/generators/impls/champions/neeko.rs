use super::*;

impl Generator for Neeko {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Initial Magic Damage", Min),
                ("Subsequent Magic Damage", _1),
                ("Total Maximum Magic Damage", Max),
            ],
        )
        .ability(Key::W, [("Bonus Magic Damage", Void)])
        .ability(Key::E, [("Magic Damage", Void)])
        .ability(Key::R, [("Magic Damage", Void)])
        .combos([
            &[
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(W(Void)),
                Ability(R(Void)),
            ],
            &[Ability(Q(_1)), Attack, Ability(W(Void))],
        ])?
        .end()
    }
}
