use super::*;

impl Generator for Seraphine {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 2 */])
            .ability(
                Key::Q,
                [
                    (0, Min), /* Magic Damage */
                    (1, Max), /* Maximum Enhanced Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(Key::R, [(1, Void) /* Magic Damage */])
            .combo([Ability(R(Void)), Ability(E(Void)), Ability(Q(Max))])?
            .combo([Ability(R(Void)), Ability(E(Void)), Ability(Q(Max)), Attack])?
            .combo([Ability(E(Void)), Ability(R(Void)), Ability(Q(Max)), Attack])?
            .combo([Ability(E(Void)), Ability(Q(Max)), Attack])?
            .combo([
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Attack,
            ])?
            .end()
    }
}
