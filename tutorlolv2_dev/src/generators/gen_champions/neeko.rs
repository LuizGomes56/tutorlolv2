use super::*;

impl Generator for Neeko {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Min), /* Initial Magic Damage */
                (2, _1),  /* Subsequent Magic Damage */
                (3, Max), /* Total Maximum Magic Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Bonus Magic Damage */])
        .ability(Key::E, [(1, Void) /* Magic Damage */])
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .combo([
            Ability(E(Void)),
            Ability(Q(Max)),
            Attack,
            Ability(W(Void)),
            Ability(R(Void)),
        ])?
        .combo([Ability(Q(_1)), Attack, Ability(W(Void))])?
        .end()
    }
}
