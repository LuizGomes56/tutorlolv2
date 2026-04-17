use super::*;

impl Generator<Champion> for Akshan {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        {
            let (passive, passive_description) = self.get_passive_description(0, 0);
            let passive_numbers = passive_description.capture_numbers_slash();
            let passive_scaling = passive_description
                .get_scalings()
                /* Remove captured shield scaling */
                .split("+")
                .next()
                .ok_or("Failed to get first passive scalling")?
                .trim()
                .to_string();

            let mut passive_damages = vec![String::new(); 18];

            [0..6, 6..11, 11..16, 16..18]
                .into_iter()
                .enumerate()
                .for_each(|(i, range)| {
                    range.for_each(|j| {
                        passive_damages[j] = passive_numbers[i].plus(&passive_scaling);
                    })
                });

            let ability = passive.format(passive_damages);
            self.insert(P(Void), ability);
        }

        self.ability(Key::Q, [(0, 0, Min), (3, 0, Max)])
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
                    .get(0)
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
            .progress(Preserve);

        self.end()
    }
}
