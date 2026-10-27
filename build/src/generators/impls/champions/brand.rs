use super::*;

impl Generator for Brand {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, Void), /* Ablaze */
                (1, _1),   /* Description 2 */
            ],
        )
        // The values based on stacks are the damage cap against monsters
        .modify(P(Void), |_| f![0.02 * EnemyMaxHealth])?
        .ability(Key::Q, [(0, Void) /* Magic Damage */])
        .ability(
            Key::W,
            [
                (0, Max), /* Increased Damage */
                (1, Min), /* Magic Damage */
            ],
        )
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, Min), /* Magic Damage */
                (2, Max), /* Total Single-Target Damage */
            ],
        )
        .combo([Ability(Q(Void)), Ability(E(Void)), Ability(W(Max))])?
        .combo([
            Ability(R(Max)),
            Ability(Q(Void)),
            Ability(E(Void)),
            Ability(W(Max)),
            Attack,
        ])?
        .combo([
            Ability(E(Void)),
            Ability(R(Max)),
            Ability(Q(Void)),
            Ability(W(Max)),
        ])?
        .combo([
            Ability(E(Void)),
            Ability(Q(Void)),
            Ability(W(Max)),
            Ability(R(Max)),
            Attack,
            Attack,
        ])?
        .combo([
            Ability(W(Max)),
            Ability(Q(Void)),
            Ability(E(Void)),
            Ability(R(Max)),
            Attack,
        ])?
        .combo([
            Ability(Q(Void)),
            Ability(E(Void)),
            Ability(W(Max)),
            Ability(R(Max)),
            Attack,
        ])?
        .combo([
            Ability(E(Void)),
            Ability(W(Max)),
            Ability(R(Max)),
            Ability(Q(Void)),
            Attack,
        ])?
        .combo([Ability(E(Void)), Ability(Q(Void)), Ability(W(Max))])?
        .combo([Ability(W(Max)), Ability(E(Void)), Ability(Q(Void))])?
        .end()
    }
}
