use super::*;

impl Generator for Teemo {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Void), /* Magic Damage On-Hit */
                    (1, Min),  /* Magic Damage per Tick */
                    (5, Max),  /* Total Poison Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (1, Min), /* Magic Damage per Tick */
                    (4, Max), /* Total Magic Damage */
                ],
            )
            .combo([
                Ability(R(Max)),
                Ability(R(Max)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
                Attack,
            ])?
            .combo([Attack, Ability(Q(Void))])?
            .combo([Ability(R(Max)), Ability(Q(Void)), Attack])?
            .combo([
                Attack,
                Ability(R(Max)),
                Attack,
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
            ])?
            .combo([Attack, Attack, Ability(Q(Void)), Attack, Attack])?
            .end()
    }
}
