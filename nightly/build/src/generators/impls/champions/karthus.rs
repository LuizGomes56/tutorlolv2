use super::*;

impl Generator for Karthus {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Isolated Enhanced Damage", Max),
                ("Magic Damage", Min),
            ],
        )
        .ability(
            Key::E,
            [
                ("Damage Per Second", Max),
                ("Magic Damage Per Tick", Min),
            ],
        )
        .ability(Key::R, [("Magic Damage", Void)])
        .combos([
            &[
                Ability(Q(Min)),
                Ability(E(Max)),
                Ability(Q(Min)),
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(R(Void)),
                Ability(Q(Min)),
            ],
            &[Ability(Q(Max)), Ability(R(Void))],
            &[
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
            ],
        ])?
        .end()
    }
}
