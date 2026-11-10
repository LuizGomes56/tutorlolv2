use super::*;

impl Generator for Yasuo {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Critical Strike Damage", Max),
                ("Physical Damage", Min),
            ],
        )
        .ability(
            Key::E,
            [
                ("Bonus Damage per Stack", _1Min),
                ("Magic Damage", Min),
                ("Maximum Bonus Damage", _1Max),
                ("Total Combined Damage", Max),
            ],
        )
        .ability(Key::R, [("Physical Damage", Void)])
        .combos([
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Max)),
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Max)),
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[
                Ability(E(Min)),
                Ability(Q(Min)),
                Ability(E(Min)),
                Ability(Q(Min)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Max)),
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[Ability(E(Min)), Attack, Ability(Q(Max)), Ability(E(Min))],
        ])?
        .end()
    }
}
