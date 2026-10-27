use super::*;

impl Generator for Zac {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Magic Damage */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability(Key::W, [(1, Void) /* Magic Damage */])
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, Min),  /* Magic Damage Per Hit */
                (1, Void), /* Reduced Damage Per Hit */
                (2, Max),  /* Total Magic Damage */
            ],
        )
        .combo([
            Ability(Q(Max)),
            Ability(E(Void)),
            Attack,
            Ability(E(Void)),
            Ability(W(Void)),
            Ability(R(Void)),
            Ability(W(Void)),
            Attack,
        ])?
        .combo([
            Ability(E(Void)),
            Ability(Q(Max)),
            Ability(E(Void)),
            Attack,
            Ability(W(Void)),
            Ability(R(Void)),
            Ability(W(Void)),
            Attack,
        ])?
        .combo([Ability(E(Void)), Ability(R(Void)), Ability(W(Void))])?
        .combo([
            Ability(Q(Max)),
            Ability(E(Void)),
            Ability(E(Void)),
            Attack,
            Ability(R(Void)),
            Ability(W(Void)),
        ])?
        .combo([
            Ability(Q(Max)),
            Ability(E(Void)),
            Attack,
            Ability(R(Void)),
            Ability(W(Void)),
            Ability(E(Void)),
            Attack,
            Ability(W(Void)),
        ])?
        .combo([
            Ability(Q(Max)),
            Ability(E(Void)),
            Attack,
            Ability(E(Void)),
            Ability(W(Void)),
            Attack,
        ])?
        .combo([
            Ability(E(Void)),
            Ability(Q(Max)),
            Ability(E(Void)),
            Attack,
            Attack,
            Ability(W(Void)),
            Attack,
        ])?
        .combo([
            Ability(E(Void)),
            Ability(W(Void)),
            Ability(Q(Max)),
            Ability(E(Void)),
            Attack,
            Attack,
            Ability(R(Void)),
            Ability(W(Void)),
            Attack,
        ])?
        .end()
    }
}
