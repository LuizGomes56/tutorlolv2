use super::*;

impl Generator for Vi {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Maximum Physical Damage */
                (1, Min), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::W, [(1, Void) /* Bonus Physical Damage */])
        .ability(Key::E, [(0, Void) /* Physical Damage */])
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .damage_types([Q(Min), Q(Max), E(Void)], Physical)?
        .combo([
            Ability(Q(Max)),
            Attack,
            Ability(E(Void)),
            Attack,
            Ability(R(Void)),
            Attack,
            Ability(E(Void)),
            Attack,
        ])?
        .combo([
            Ability(Q(Max)),
            Attack,
            Ability(E(Void)),
            Attack,
            Ability(E(Void)),
            Attack,
            Ability(R(Void)),
        ])?
        .combo([Ability(Q(Max)), Ability(R(Void))])?
        .combo([
            Ability(R(Void)),
            Attack,
            Ability(E(Void)),
            Attack,
            Ability(Q(Max)),
            Attack,
            Ability(E(Void)),
            Attack,
        ])?
        .combo([
            Attack,
            Ability(E(Void)),
            Attack,
            Ability(R(Void)),
            Attack,
            Ability(E(Void)),
            Ability(Q(Max)),
            Attack,
            Attack,
        ])?
        .combo([
            Attack,
            Ability(E(Void)),
            Attack,
            Ability(Q(Max)),
            Attack,
            Ability(E(Void)),
            Attack,
        ])?
        .combo([Ability(Q(Max)), Attack, Ability(E(Void)), Attack, Attack])?
        .end()
    }
}
