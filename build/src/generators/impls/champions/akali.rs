use super::*;

impl Generator for Akali {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                ("Swinging Kama", _1),
                ("Swinging Kama [1]", _2),
            ],
        )
        .merge_sum([P(_1), P(_2)], P(Void))?
        .ability(Key::Q, [("Magic Damage", Void)])
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
                ("Magic Damage", _1),
                ("Maximum Magic Damage", Max),
                ("Minimum Magic Damage", Min),
            ],
        )
        .combos([
            &[
                Ability(R(_1)),
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Min)),
            ],
            &[
                Ability(R(_1)),
                Ability(E(Min)),
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
                Ability(R(Min)),
            ],
            &[
                Ability(R(_1)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Min)),
            ],
            &[Ability(E(Min)), Ability(E(Min)), Ability(Q(Void)), Attack],
            &[
                Ability(R(_1)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Min)),
            ],
            &[
                Ability(Q(Void)),
                Attack,
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
            ],
            &[
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ],
            &[
                Ability(Q(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
