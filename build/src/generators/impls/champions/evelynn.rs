use super::*;

impl Generator for Evelynn {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Min), /* Magic Damage */
                (4, Max), /* Total Magic Damage */
                (0, _1),  /* Bonus Magic Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Bonus Magic Damage */])
        .ability_nth(1, Key::E, [(1, Void) /* Magic Damage */])
        .modify(E(Void), |_| {
            f![60 + 30 * (ELevel - 1) + (0.03 + 0.00015 * AbilityPower) * EnemyMaxHealth]
        })?
        .ability(Key::E, [(0, _1) /* Empowered Magic Damage */])
        .modify(E(_1), |_| {
            f![80 + 40 * (ELevel - 1) + (0.04 + 0.00025 * AbilityPower) * EnemyMaxHealth]
        })?
        .ability(
            Key::R,
            [
                (1, Min), /* Magic Damage */
                (0, Max), /* Empowered Damage */
            ],
        )
        .combos([
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(R(Max)),
            ],
            &[Ability(Q(Max)), Ability(E(Void)), Ability(R(Max))],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(W(Void)),
                Ability(R(Max)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(R(Max)),
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Ability(R(Max)),
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
            ],
        ])?
        .end()
    }
}
