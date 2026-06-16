use super::*;

impl Generator for Smolder {
    fn generate(&mut self) -> MayFail {
        let passive_q = self
            .formula(Key::P, 4)?
            .times(Stacks)
            .times(MagicMultiplier);
        let passive_w = self.formula(Key::P, 2)?.times(MagicMultiplier);
        let passive_e = self.formula(Key::P, 3)?.times(MagicMultiplier);

        let q_tier_3 = f![(0.083 * BonusAd / 100 + 0.017 * Stacks / 100) * EnemyMaxHealth];

        let w_dmg = |dmg: &str| f![(dmg * PhysicalMultiplier) + passive_w];

        self.ability(Key::Q, [(0, Min) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, Void), /* Explosion Physical Damage */
                    (1, Min),  /* Glob Physical Damage */
                    (2, Max),  /* Total Physical Damage On Champion Hit */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Max), /* Increased Physical Damage */
                    (1, Min), /* Physical Damage */
                ],
            )
            .modify(Q(Min), |dmg| {
                f![(dmg * PhysicalMultiplier) + (passive_q) + q_tier_3]
            })?
            .clone_with(Q(Min), Q(Max), |dmg| f![2 * (dmg)])?
            .modify(W(Void), w_dmg)?
            .modify(W(Min), w_dmg)?
            .modify(W(Max), w_dmg)?
            .insert(
                E(Min),
                Ability {
                    name: "Flap, Flap, Flap".into(),
                    damage_type: Mixed,
                    attributes: Undefined,
                    comment: "Physical Damage per Hit".into(),
                    damage: f![
                        (10 + 5 * ELevel + 0.3 * AttackDamage) * PhysicalMultiplier + (passive_e)
                    ],
                },
            )
            .clone_with(E(Min), E(Max), |dmg| {
                let hits = f![5 + Stacks / 100];
                f![(dmg) * (hits)]
            })?
            .damage_types([Q(Min), Q(Max), W(Void), W(Min), W(Max)], Mixed)?
            .damage_types([R(Min), R(Max)], Physical)?
            .end()
    }
}
