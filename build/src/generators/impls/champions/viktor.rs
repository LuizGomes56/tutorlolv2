use super::*;

impl Generator for Viktor {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage", Min),
                ("Modified Magic Damage", Void),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(
            Key::E,
            [
                ("Magic Damage", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(
            Key::R,
            [
                ("Magic Damage", Void),
                ("Magic Damage Per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .combos([
            &[
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
                Attack,
            ],
            &[
                Ability(Q(Void)),
                Attack,
                Ability(E(Max)),
                Ability(R(Void)),
                Attack,
                Attack,
            ],
            &[
                Ability(R(Void)),
                Ability(Q(Void)),
                Attack,
                Ability(E(Max)),
                Attack,
                Attack,
                Attack,
            ],
            &[
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(E(Max)),
                Ability(R(Void)),
                Attack,
                Attack,
                Attack,
            ],
            &[Ability(Q(Void)), Attack, Ability(E(Max))],
        ])?
        .end()
    }
}
