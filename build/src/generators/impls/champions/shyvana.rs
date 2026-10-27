use super::*;

impl Generator for Shyvana {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Void), /* Area Physical Damage */
                (1, _1),   /* True Damage */
            ],
        )
        .ability(Key::W, [(1, Void) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, Max),  /* Increased/Explosion Magic Damage */
                (1, Void), /* Magic Damage */
                (2, Min),  /* Subsequent Explosion Damage */
            ],
        )
        .ability(Key::R, [(4, Void) /* Magic Damage */])
        .damage_type(Q(Void), Physical)?
        .damage_type(Q(_1), True)?
        .combo([
            Ability(R(Void)),
            Ability(E(Void)),
            Ability(Q(Void)),
            Ability(E(Void)),
            Attack,
        ])?
        .combo([
            Ability(W(Void)),
            Ability(E(Void)),
            Ability(Q(Void)),
            Ability(E(Void)),
            Attack,
        ])?
        .combo([
            Ability(W(Void)),
            Attack,
            Ability(E(Void)),
            Ability(Q(Void)),
            Ability(E(Void)),
            Attack,
            Attack,
        ])?
        .combo([
            Ability(E(Void)),
            Ability(W(Void)),
            Attack,
            Ability(Q(Void)),
            Attack,
        ])?
        .combo([
            Ability(W(Void)),
            Ability(R(Void)),
            Ability(E(Void)),
            Ability(Q(Void)),
            Ability(E(Void)),
            Attack,
        ])?
        .combo([
            Ability(R(Void)),
            Ability(E(Void)),
            Ability(W(Void)),
            Attack,
            Ability(Q(Void)),
            Attack,
        ])?
        .end()
    }
}
