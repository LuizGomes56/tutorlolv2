use super::*;

impl Generator for Smolder {
    fn generate(&mut self) -> MayFail {
        let passive_q = self
            .formula(Key::P, "Innate [4]")?
            .times(Stacks)
            .times(MagicMultiplier);
        let passive_w = self.formula(Key::P, "Innate [2]")?.times(MagicMultiplier);
        let passive_e = self.formula(Key::P, "Innate [3]")?.times(MagicMultiplier);

        let q_tier_3 = f![(0.083 * BonusAd / 100 + 0.017 * Stacks / 100) * EnemyMaxHealth];

        let w_dmg = |dmg: &str| f![((dmg) * PhysicalMultiplier) + passive_w];

        self.ability(Key::Q, [("Physical Damage", Min)])
            .ability(
                Key::W,
                [
                    ("Explosion Physical Damage", Void),
                    ("Glob Physical Damage", Min),
                    ("Total Physical Damage On Champion Hit", Max),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Increased Physical Damage", Max),
                    ("Physical Damage", Min),
                ],
            )
            .modify(Q(Min), |dmg| {
                f![((dmg) * PhysicalMultiplier) + (passive_q) + q_tier_3]
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
            .combos([
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Max)),
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Attack, Ability(W(Void)), Attack, Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
