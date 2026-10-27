use super::*;

impl Generator for Nilah {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Maximum Physical Damage */
                (1, Min), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::E, [(0, Void) /* Physical Damage */])
        .ability(
            Key::R,
            [
                (0, _2),  /* Burst Physical Damage */
                (1, _1),  /* Maximum Total Physical Damage */
                (2, Min), /* Physical Damage per Tick */
                (3, Max), /* Total Physical Damage */
            ],
        )
        .combo([
            Ability(Q(Min)),
            Attack,
            Ability(E(Void)),
            Ability(R(Max)),
            Attack,
            Attack,
            Ability(Q(Min)),
            Attack,
            Ability(E(Void)),
        ])?
        .combo([Ability(E(Void)), Ability(R(Max))])?
        .combo([Ability(E(Void)), Ability(R(Max)), Ability(E(Void))])?
        .combo([
            Ability(E(Void)),
            Ability(E(Void)),
            Ability(R(Max)),
            Ability(Q(Max)),
            Attack,
            Attack,
            Attack,
        ])?
        .combo([
            Ability(E(Void)),
            Ability(Q(Max)),
            Attack,
            Ability(R(Max)),
            Attack,
            Attack,
            Ability(E(Void)),
            Attack,
        ])?
        .combo([
            Ability(E(Void)),
            Attack,
            Ability(E(Void)),
            Ability(Q(Max)),
            Ability(R(Max)),
        ])?
        .combo([
            Ability(E(Void)),
            Ability(Q(Min)),
            Attack,
            Attack,
            Attack,
            Attack,
            Ability(E(Void)),
            Ability(Q(Min)),
        ])?
        .combo([Ability(E(Void)), Ability(Q(Max)), Attack, Attack, Attack])?
        .end()
    }
}
