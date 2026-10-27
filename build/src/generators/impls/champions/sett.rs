use super::*;

impl Generator for Sett {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 2 */])
            .ability(
                Key::Q,
                [
                    (0, Min), /* Bonus Physical Damage */
                    (1, Max), /* Total Bonus Physical Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Physical Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .damage_types([P(Void), W(Void), R(Min), R(Max)], Physical)?
            .combo([Ability(Q(Max)), Ability(E(Void)), Ability(R(Max))])?
            .combo([
                Ability(R(Max)),
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(Q(Max)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(R(Max)),
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(W(Void)),
            ])?
            .combo([
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(R(Max)),
                Attack,
                Attack,
                Ability(W(Void)),
            ])?
            .combo([Attack, Attack, Ability(Q(Max)), Attack, Attack])?
            .combo([
                Attack,
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(W(Void)),
            ])?
            .combo([
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(W(Void)),
            ])?
            .combo([
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(W(Void)),
            ])?
            .end()
    }
}
