use super::*;

impl Generator for Sion {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(1, Key::P, [(0, Void) /* Description 1 */])
            .modify(P(Void), |dmg| f![(dmg) / 10])?
            .damage_type(P(Void), Physical)?
            .comment(P(Void), "Glory in Death bonus damage on-hit")?
            .ability(
                Key::Q,
                [
                    (3, Max), /* Maximum Physical Damage */
                    (6, Min), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Maximum Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .combo([
                Ability(R(Max)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(W(Void)),
            ])?
            .combo([
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Ability(R(Max)),
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(R(Max)),
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(W(Void)),
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Ability(R(Max)),
                Attack,
                Attack,
                Ability(W(Void)),
            ])?
            .end()
    }
}
