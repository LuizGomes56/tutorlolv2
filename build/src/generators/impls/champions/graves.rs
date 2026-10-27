use super::*;

impl Generator for Graves {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Physical Damage */
                (1, Max), /* Total Physical Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, Void), /* Physical Damage */
                (1, _1),   /* Reduced Damage */
            ],
        )
        .combo([
            Ability(W(Void)),
            Attack,
            Ability(Q(Max)),
            Attack,
            Ability(R(Void)),
            Attack,
        ])?
        .combo([
            Ability(W(Void)),
            Ability(Q(Max)),
            Ability(R(Void)),
            Attack,
            Attack,
            Attack,
        ])?
        .combo([Ability(Q(Max)), Attack, Ability(R(Void))])?
        .combo([Ability(W(Void)), Ability(Q(Max)), Ability(R(Void))])?
        .combo([
            Ability(W(Void)),
            Attack,
            Ability(Q(Max)),
            Attack,
            Ability(R(Void)),
        ])?
        .combo([
            Ability(W(Void)),
            Ability(Q(Max)),
            Attack,
            Attack,
            Ability(R(Void)),
            Attack,
        ])?
        .combo([Ability(W(Void)), Attack, Ability(Q(Max)), Attack, Attack])?
        .combo([Attack, Attack, Ability(Q(Max)), Attack])?
        .combo([Ability(W(Void)), Attack, Attack, Ability(Q(Max)), Attack])?
        .end()
    }
}
