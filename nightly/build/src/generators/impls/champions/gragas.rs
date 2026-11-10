use super::*;

impl Generator for Gragas {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Maximum Magic Damage", Max),
                ("Minimum Magic Damage", Min),
            ],
        )
        .ability(Key::W, [("Bonus Magic Damage", Void)])
        .ability(Key::E, [("Magic Damage", Void)])
        .ability(Key::R, [("Magic Damage", Void)])
        .combos([
            &[
                Ability(W(Void)),
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(R(Void)),
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
            ],
            &[Ability(E(Void)), Ability(R(Void))],
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
                Ability(Q(Min)),
            ],
            &[Ability(Q(Max)), Ability(W(Void))],
        ])?
        .end()
    }
}
