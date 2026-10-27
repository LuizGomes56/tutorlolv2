use super::*;

impl Generator for Sivir {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min),  /* Minimum Damage */
                (1, Void), /* Physical Damage */
                (2, Max),  /* Total Maximum Champion Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (1, Max), /* Bounce Critical Damage */
                (2, Min), /* Bounce Damage */
            ],
        )
        .combo([Ability(Q(Void)), Attack, Ability(W(Max)), Attack, Attack])?
        .combo([Attack, Ability(W(Max)), Ability(Q(Void)), Attack, Attack])?
        .combo([
            Attack,
            Ability(W(Max)),
            Attack,
            Ability(Q(Void)),
            Attack,
            Attack,
        ])?
        .combo([Attack, Ability(W(Max)), Ability(Q(Void))])?
        .combo([Attack, Ability(Q(Void)), Attack])?
        .end()
    }
}
