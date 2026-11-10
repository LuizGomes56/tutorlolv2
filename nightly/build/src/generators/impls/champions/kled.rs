use super::*;

impl Generator for Kled {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(
            1,
            Key::Q,
            [
                ("Physical Damage", Min),
                ("Total Physical Damage", Max),
            ],
        )
        .ability(
            Key::Q,
            [
                ("Physical Damage", _1Min),
                ("Maximum Damage", _1Max),
            ],
        )
        .comment(Q(_1Min), "Pocket Pistol while dismounted")?
        .comment(Q(_1Max), "Pocket Pistol while dismounted, maximum damage")?
        .ability(Key::W, [("Additional Physical Damage", Void)])
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
                ("Physical Damage", Min),
                ("Total Physical Damage", Max),
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
                ("Minimum Magic Damage", Min),
                ("Maximum Magic Damage", Max),
            ],
        )
        .modify(R(Min), |_| {
            f![(0.04 + 0.01 * (RLevel - 1) + 0.0003 * BonusAd) * EnemyMaxHealth]
        })?
        .modify(R(Max), |_| {
            f![(0.12 + 0.03 * (RLevel - 1) + 0.0009 * BonusAd) * EnemyMaxHealth]
        })?
        .damage_types([R(Min), R(Max)], Magic)?
        .combos([
            &[
                Ability(R(Max)),
                Ability(E(Min)),
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
                Attack,
                Attack,
                Attack,
            ],
            &[
                Ability(E(Min)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
                Ability(E(Min)),
                Attack,
            ],
            &[
                Ability(Q(Max)),
                Attack,
                Ability(E(Min)),
                Attack,
                Attack,
                Ability(E(Min)),
                Attack,
            ],
            &[
                Ability(E(Min)),
                Ability(Q(Max)),
                Attack,
                Ability(R(Max)),
                Attack,
                Attack,
                Ability(E(Min)),
                Attack,
            ],
            &[
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
            ],
            &[
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
            ],
            &[
                Ability(Q(Max)),
                Ability(E(Min)),
                Attack,
                Attack,
                Attack,
                Ability(E(Min)),
                Attack,
            ],
            &[
                Ability(E(Max)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
                Attack,
            ],
        ])?
        .end()
    }
}
