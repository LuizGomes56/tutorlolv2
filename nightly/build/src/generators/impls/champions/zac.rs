use super::*;

impl Generator for Zac {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(Key::W, [("Magic Damage", Void)])
        .ability(Key::E, [("Magic Damage", Void)])
        .ability(
            Key::R,
            [
                ("Magic Damage Per Hit", Min),
                ("Reduced Damage Per Hit", Void),
                ("Total Magic Damage", Max),
            ],
        )
        .combos([
            &[
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(R(Void)),
                Ability(W(Void)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Ability(R(Void)),
                Ability(W(Void)),
                Attack,
            ],
            &[Ability(E(Void)), Ability(R(Void)), Ability(W(Void))],
            &[
                Ability(Q(Max)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
                Ability(W(Void)),
            ],
            &[
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
            ],
            &[
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(W(Void)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(R(Void)),
                Ability(W(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
