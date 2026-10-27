use super::*;

impl Generator for Kled {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(
            1,
            Key::Q,
            [
                (1, Min), /* Physical Damage */
                (3, Max), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::Q,
            [
                (1, _1Min), /* Physical Damage */
                (0, _1Max), /* Maximum Damage */
            ],
        )
        .comment(Q(_1Min), "Pocket Pistol while dismounted")?
        .comment(Q(_1Max), "Pocket Pistol while dismounted, maximum damage")?
        .ability(Key::W, [(0, Void) /* Additional Physical Damage */])
        .modify(W(Void), |_| {
            f![20
                + 10 * (WLevel - 1)
                + (0.045 + 0.005 * (WLevel - 1) + 0.0002 * BonusAd + 0.00004 * BonusHealth)
                    * EnemyMaxHealth]
        })?
        .ability_nth(
            1,
            Key::E,
            [
                (0, Min), /* Physical Damage */
                (1, Max), /* Total Physical Damage */
            ],
        )
        .damage_types(
            [Q(Min), Q(Max), Q(_1Min), Q(_1Max), W(Void), E(Min), E(Max)],
            Physical,
        )?
        .ability_nth(
            1,
            Key::R,
            [
                (2, Min), /* Minimum Magic Damage */
                (1, Max), /* Maximum Magic Damage */
            ],
        )
        .modify(R(Min), |_| {
            f![(0.04 + 0.01 * (RLevel - 1) + 0.0003 * BonusAd) * EnemyMaxHealth]
        })?
        .modify(R(Max), |_| {
            f![(0.12 + 0.03 * (RLevel - 1) + 0.0009 * BonusAd) * EnemyMaxHealth]
        })?
        .damage_types([R(Min), R(Max)], Magic)?
        .combo([
            Ability(R(Max)),
            Ability(E(Min)),
            Ability(Q(Max)),
            Attack,
            Ability(E(Min)),
            Attack,
            Attack,
            Attack,
        ])?
        .combo([
            Ability(E(Min)),
            Ability(Q(Max)),
            Attack,
            Attack,
            Attack,
            Ability(E(Min)),
            Attack,
        ])?
        .combo([
            Ability(Q(Max)),
            Attack,
            Ability(E(Min)),
            Attack,
            Attack,
            Ability(E(Min)),
            Attack,
        ])?
        .combo([
            Ability(E(Min)),
            Ability(Q(Max)),
            Attack,
            Ability(R(Max)),
            Attack,
            Attack,
            Ability(E(Min)),
            Attack,
        ])?
        .combo([
            Ability(R(Max)),
            Ability(Q(Max)),
            Attack,
            Attack,
            Attack,
            Attack,
            Ability(E(Min)),
            Attack,
            Ability(E(Min)),
            Attack,
        ])?
        .combo([
            Attack,
            Attack,
            Attack,
            Attack,
            Attack,
            Ability(Q(Max)),
            Ability(E(Min)),
            Attack,
            Attack,
            Ability(E(Min)),
        ])?
        .combo([
            Ability(Q(Max)),
            Ability(E(Min)),
            Attack,
            Attack,
            Attack,
            Ability(E(Min)),
            Attack,
        ])?
        .combo([
            Ability(E(Max)),
            Ability(Q(Max)),
            Attack,
            Attack,
            Attack,
            Attack,
        ])?
        .end()
    }
}
