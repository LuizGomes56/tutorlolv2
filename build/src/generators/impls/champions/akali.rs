use super::*;

impl Generator for Akali {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1), /* Swinging Kama */
                (3, _2), /* Swinging Kama [1] */
            ],
        )
        .merge_sum([P(_1), P(_2)], P(Void))?
        .ability(Key::Q, [(0, Void) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, Min), /* Magic Damage */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1),  /* Magic Damage */
                (1, Max), /* Maximum Magic Damage */
                (2, Min), /* Minimum Magic Damage */
            ],
        )
        .combo([
            Ability(R(Min)),
            Ability(E(Min)),
            Ability(E(Min)),
            Attack,
            Ability(Q(Void)),
            Ability(R(Min)),
        ])?
        .combo([
            Ability(R(Min)),
            Ability(E(Min)),
            Ability(Q(Void)),
            Ability(E(Min)),
            Attack,
            Ability(R(Min)),
        ])?
        .combo([
            Ability(R(Min)),
            Attack,
            Ability(Q(Void)),
            Attack,
            Ability(E(Min)),
            Ability(E(Min)),
            Attack,
            Ability(Q(Void)),
            Ability(R(Min)),
        ])?
        .combo([Ability(E(Min)), Ability(E(Min)), Ability(Q(Void)), Attack])?
        .combo([
            Ability(R(Min)),
            Ability(E(Min)),
            Attack,
            Ability(Q(Void)),
            Ability(E(Min)),
            Attack,
            Ability(Q(Void)),
            Ability(R(Min)),
        ])?
        .combo([
            Ability(Q(Void)),
            Attack,
            Ability(E(Min)),
            Ability(E(Min)),
            Attack,
        ])?
        .combo([
            Ability(E(Min)),
            Ability(E(Min)),
            Attack,
            Ability(Q(Void)),
            Attack,
            Ability(Q(Void)),
            Attack,
        ])?
        .combo([
            Ability(Q(Void)),
            Attack,
            Ability(Q(Void)),
            Attack,
            Ability(E(Min)),
            Ability(E(Min)),
            Attack,
            Ability(Q(Void)),
            Attack,
        ])?
        .end()
    }
}
