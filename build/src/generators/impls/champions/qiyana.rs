use super::*;

impl Generator for Qiyana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, _1) /* Innate */, (2, _2) /* Innate [1] */])
            .merge_sum([P(_1), P(_2)], P(Void))?
            .ability_nth(
                1,
                Key::Q,
                [
                    (0, Min), /* Physical Damage */
                    (1, _1),  /* Reduced Damage */
                ],
            )
            .ability(
                Key::Q,
                [
                    (0, Max), /* Increased Damage */
                    (1, _2),  /* Physical Damage */
                    (3, _3),  /* Subsequent Increased Damage */
                ],
            )
            .ability(Key::W, [(1, Void) /* Bonus Magic Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(Key::R, [(1, Void) /* Physical Damage */])
            .damage_types([P(Void), E(Void), R(Void)], Physical)?
            .combo([
                Ability(R(Void)),
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(R(Void)),
                Ability(Q(Min)),
            ])?
            .combo([
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
            ])?
            .combo([
                Ability(E(Void)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(Q(Min)),
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(Q(Min)),
            ])?
            .combo([
                Ability(E(Void)),
                Ability(R(Void)),
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(Q(Min)),
                Attack,
            ])?
            .combo([Ability(W(Void)), Ability(E(Void)), Ability(Q(Max)), Attack])?
            .combo([Ability(Q(Min)), Ability(W(Void)), Ability(Q(Min))])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
            ])?
            .end()
    }
}
