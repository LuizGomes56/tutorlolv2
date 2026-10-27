use super::*;

impl Generator for LeeSin {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(1, Key::Q, [(0, _1) /* Physical Damage */])
            .ability(
                Key::Q,
                [
                    (0, Max), /* Maximum Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .ability_nth(1, Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Collision Physical Damage */
                    (1, Min), /* Physical Damage */
                ],
            )
            .combo([Ability(Q(Min)), Ability(Q(Min)), Ability(R(Max))])?
            .combo([
                Ability(Q(Min)),
                Ability(R(Max)),
                Ability(Q(Min)),
                Ability(E(Void)),
            ])?
            .combo([
                Ability(Q(Min)),
                Ability(R(Max)),
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([Ability(Q(Min)), Ability(R(Max)), Ability(Q(Min)), Attack])?
            .combo([
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(R(Max)),
                Ability(Q(Max)),
            ])?
            .combo([
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
            ])?
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(Q(Min)),
            ])?
            .end()
    }
}
