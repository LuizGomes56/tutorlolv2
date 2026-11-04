use super::*;

impl Generator for Swain {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Bonus Damage Per Bolt", Void),
                ("Magic Damage", Min),
                ("Total Damage", Max),
            ],
        )
        .ability(Key::W, [("Magic Damage", Void)])
        .ability(Key::E, [("Magic Damage", Void)])
        .ability_nth(1, Key::R, [("Magic Damage per Tick", Void)])
        .ability(Key::R, [("Magic Damage", _1)])
        .combos([
            &[
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Ability(R(_1)),
            ],
            &[
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
                Ability(R(_1)),
            ],
            &[
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(R(_1)),
            ],
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
                Attack,
                Ability(R(_1)),
                Attack,
                Ability(Q(Void)),
            ],
            &[
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Void)),
            ],
            &[
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
