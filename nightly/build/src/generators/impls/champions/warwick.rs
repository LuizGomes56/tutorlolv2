use super::*;

impl Generator for Warwick {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", _1), ("Innate [1]", _2)])
            .merge_sum([P(_1), P(_2)], P(Void))?
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::R, [("Total Magic Damage", Void)])
            .combos([
                &[Ability(Q(Void)), Attack, Ability(R(Void))],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(R(Void)), Attack, Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void)), Attack, Attack, Attack],
            ])?
            .end()
    }
}
