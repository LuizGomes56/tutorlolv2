use super::*;

impl Generator for Riven {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .modify(P(Void), |_| {
                f![(30 + 15 / 17 * (Level - 1)) / 100 * AttackDamage]
            })?
            .ability(
                Key::Q,
                [
                    (0, Min), /* Physical Damage */
                    (1, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Physical Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Maximum Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .damage_types([P(Void), Q(Min), Q(Max), W(Void), R(Min), R(Max)], Physical)?
            .combo([
                Attack,
                Ability(W(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ])?
            .combo([
                Ability(R(Min)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(R(Min)),
                Ability(Q(Min)),
            ])?
            .combo([
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(R(Min)),
                Ability(Q(Min)),
                Attack,
                Ability(R(Min)),
                Ability(W(Void)),
            ])?
            .combo([
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(W(Void)),
            ])?
            .combo([
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ])?
            .combo([Ability(W(Void)), Ability(Q(Max))])?
            .combo([Ability(R(Max)), Ability(Q(Max))])?
            .combo([Ability(R(Max)), Ability(W(Void))])?
            .end()
    }
}
