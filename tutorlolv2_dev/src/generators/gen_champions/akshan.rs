use super::*;

impl Generator for Akshan {
    fn generate(&mut self) -> MayFail {
        let (passive, desc) = self.get_passive_description(0, 0);
        let level_scaling = &desc.capture_numbers_slash()[..4];
        let full_scaling = desc.get_scalings();
        let dmg_scaling = full_scaling
            /* Remove captured shield scaling */
            .split_once(" + ")
            .ok_or("Failed to get first passive scalling")?
            .0
            .trim();

        let mut passive_dmg = [const { String::new() }; 18];

        [0..6, 6..11, 11..16, 16..18]
            .into_iter()
            .enumerate()
            .for_each(|(i, range)| {
                let value = level_scaling[i].plus(dmg_scaling);
                passive_dmg[range].fill(value);
            });

        let ability = passive.format(passive_dmg);

        let p1_scaling = self
            .get_meraki_ability(Key::P, 0)
            .effects
            .get(1)
            .and_then(|effect| effect.description.capture_percent(0).ok())
            .unwrap_or(0.5);

        self.insert(P(Void), ability)
            .ability_raw::<18>(P(_1), |_| p1_scaling.times(AttackDamage))?
            .ability(Key::Q, [(0, 0, Min), (3, 0, Max)])
            .ability(Key::E, [(4, 0, Void)])
            .ability(
                Key::R,
                [(4, 0, _2Max), (4, 1, _1Max), (4, 2, _2Min), (4, 3, _1Min)],
            );

        for key in [_1Min, _1Max, _2Min, _2Max] {
            let ability = self.get_mut(AbilityId::R(key))?;

            for damage in ability.damage.iter_mut() {
                let base_scalings = *damage
                    .capture_numbers::<f64>()
                    .first()
                    .ok_or("Can't capture first scaling")?;
                let norm_scaling = damage.capture_parens(0)?;
                *damage = base_scalings
                    .plus(norm_scaling)
                    .parens()
                    .times(1.plus(CritChance).div(200).parens());
            }
        }

        self.attr(Area, [Q(Min), Q(Max)])?
            .damage_type(P(Void), Magic)?
            .combo([Ability(Q(Min)), Attack, Ability(P(_1)), Ability(Q(Min))])?
            .progress(Stable)
            .end()
    }
}
