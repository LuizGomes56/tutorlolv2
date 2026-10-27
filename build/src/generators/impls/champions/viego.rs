use super::*;

impl Generator for Viego {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Void), /* Bonus Physical Damage */
                (2, Min),  /* Minimum Bonus Damage */
                (3, Max),  /* Physical Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .combo([
            Ability(W(Void)),
            Attack,
            Ability(Q(Void)),
            Ability(R(Void)),
            Attack,
        ])?
        .combo([Ability(W(Void)), Attack, Ability(Q(Void)), Attack])?
        .combo([Ability(Q(Void)), Attack, Ability(W(Void)), Attack])?
        .combo([Ability(W(Void)), Attack, Ability(Q(Void))])?
        .combo([
            Ability(Q(Void)),
            Attack,
            Ability(W(Void)),
            Attack,
            Ability(R(Void)),
            Attack,
        ])?
        .combo([Ability(W(Void)), Ability(Q(Void))])?
        .end()
    }
}
