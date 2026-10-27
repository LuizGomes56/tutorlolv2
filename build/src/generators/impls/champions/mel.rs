use super::*;

impl Generator for Mel {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Initial Explosion Magic Damage */
                (3, Max), /* Total Magic Damage */
                (1, _1),  /* Magic Damage per Subsequent Explosion */
            ],
        )
        .ability(
            Key::E,
            [
                (3, Void), /* Orb Magic Damage */
                (0, _1),   /* Field Magic Damage per Second */
            ],
        )
        .ability(Key::R, [(1, Void) /* Magic Damage */])
        .modify(R(Void), |_| {
            f![125
                + 37.5 * (RLevel - 1)
                + 0.3 * AbilityPower
                + Stacks * (4 + 1.5 * (RLevel - 1) + 0.04 * AbilityPower)]
        })?
        .comment(
            R(Void),
            "Magic damage, Stacks = Overwhelm stacks on the target",
        )?
        .combos([
            &[
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[Ability(E(Void)), Ability(Q(Max)), Ability(R(Void))],
            &[
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
        ])?
        .end()
    }
}
