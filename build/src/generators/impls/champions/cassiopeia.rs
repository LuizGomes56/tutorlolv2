use super::*;

impl Generator for Cassiopeia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage Per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(
            Key::W,
            [
                ("Magic Damage Per Second", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(
            Key::E,
            [
                ("Bonus Magic Damage", Min),
                ("Total Enhanced Damage", Max),
            ],
        )
        .ability(Key::R, [("Magic Damage", Void)])
        .combos([
            &[
                Ability(R(Void)),
                Ability(W(Max)),
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(Q(Max)),
                Ability(E(Min)),
                Ability(E(Min)),
            ],
            &[
                Ability(Q(Max)),
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(W(Max)),
                Ability(E(Min)),
                Ability(R(Void)),
                Ability(E(Min)),
                Ability(Q(Max)),
                Ability(E(Min)),
            ],
            &[
                Ability(R(Void)),
                Ability(W(Max)),
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(Q(Max)),
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(E(Min)),
            ],
            &[
                Ability(Q(Max)),
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(W(Max)),
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(Q(Max)),
                Ability(E(Min)),
            ],
            &[
                Ability(Q(Max)),
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(E(Min)),
                Ability(Q(Max)),
            ],
        ])?
        .end()
    }
}
