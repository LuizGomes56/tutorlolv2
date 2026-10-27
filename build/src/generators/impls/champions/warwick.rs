use super::*;

impl Generator for Warwick {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Innate */, (1, _2) /* Innate [1] */])
            .merge_sum([P(_1), P(_2)], P(Void))?
            .ability(Key::Q, [(2, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Total Magic Damage */])
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
