use super::*;

impl Generator for Qiyana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", _1), ("Innate [1]", _2)])
            .merge_sum([P(_1), P(_2)], P(Void))?
            .map_sum(P(Void), [(Key::P, "Innate"), (Key::P, "Innate [1]")])?
            .ability_nth(
                1,
                Key::Q,
                [
                    ("Physical Damage", Min),
                    ("Reduced Damage", _1),
                ],
            )
            .ability(
                Key::Q,
                [
                    ("Increased Damage", Max),
                    ("Physical Damage", _2),
                    ("Subsequent Increased Damage", _3),
                ],
            )
            .ability(Key::W, [("Bonus Magic Damage", Void)])
            .ability(Key::E, [("Physical Damage", Void)])
            .ability(Key::R, [("Physical Damage", Void)])
            .damage_types([P(Void), E(Void), R(Void)], Physical)?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(E(Void)), Ability(Q(Max)), Attack],
                &[Ability(Q(Min)), Ability(W(Void)), Ability(Q(Min))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
            ])?
            .end()
    }
}
