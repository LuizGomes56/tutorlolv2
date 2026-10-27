use super::*;

impl Generator for Hecarim {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, Void) /* Physical Damage */])
            .modify(Q(Void), |dmg| {
                /* Rampage bonus damage per stack */
                f![(dmg) * Stacks * (1.03 + 0.03 * BonusAd / 100)]
            })?
            .ability(
                Key::W,
                [
                    (2, Min), /* Magic Damage Per Tick */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Max), /* Maximum Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Magic damage */])
            .combo([
                Ability(E(Min)),
                Ability(W(Max)),
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
            ])?
            .combo([
                Ability(W(Max)),
                Attack,
                Ability(E(Max)),
                Ability(R(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Ability(R(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Ability(R(Void)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Ability(W(Max)),
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ])?
            .end()
    }
}
