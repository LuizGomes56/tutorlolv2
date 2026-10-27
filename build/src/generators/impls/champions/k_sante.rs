use super::*;

impl Generator for KSante {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .modify(P(Void), |_| {
                f![12 + (0.01 + 0.01 / 17 * (Level - 1)) * EnemyMaxHealth]
            })?
            .damage_type(P(Void), Physical)?
            .comment(P(Void), "Mark consumption bonus damage")?
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (3, Min), /* Physical Damage */
                    (4, Max), /* Total Maximum Mixed Damage */
                ],
            )
            .modify(W(Min), |_| {
                f![45
                    + 30 * (WLevel - 1)
                    + (0.08 + 0.0002 * BonusArmor + 0.0002 * BonusMagicResist) * EnemyMaxHealth]
            })?
            .modify(W(Max), |_| {
                f![81
                    + 54 * (WLevel - 1)
                    + (0.144 + 0.00036 * BonusArmor + 0.00036 * BonusMagicResist) * EnemyMaxHealth]
            })?
            .damage_type(W(Max), Mixed)?
            .ability(
                Key::R,
                [
                    (2, Min), /* Strike Physical Damage */
                    (3, Max), /* Total Physical Damage */
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
