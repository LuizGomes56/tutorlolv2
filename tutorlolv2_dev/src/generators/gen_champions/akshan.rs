use super::*;

// #![stable]

impl Generator<Champion> for Akshan {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        {
            let (passive, passive_description) = self.get_passive_description(0, 0);
            let passive_numbers = passive_description.capture_numbers_slash();
            let passive_scaling = passive_description
                .get_scalings()
                .split("+")
                .next()
                .ok_or("Failed to get first passive scalling")?
                .trim()
                .to_string();

            let get_passive_damage = |base| format!("{base} + {passive_scaling}");
            let mut passive_damages = vec![String::new(); 18];

            [0..6, 6..11, 11..16, 16..18]
                .into_iter()
                .enumerate()
                .for_each(|(i, range)| {
                    range.for_each(|j| {
                        passive_damages[j] = get_passive_damage(passive_numbers[i]);
                    })
                });

            let ability = passive.format(passive_damages);
            self.insert(P::Void, ability);
        }

        self.ability(Q, [(0, 0, Min), (3, 0, Max)]);
        self.ability(E, [(4, 0, Void)]);
        self.ability(
            R,
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
                *damage = format!(
                    "({base_scalings} + {norm_scaling}) * {crit_scaling}",
                    crit_scaling = format_args!("(1 + {CritChance} / 200)")
                );
            }
        }

        self.attr(Area, [Q::Min, Q::Max])?;
        self.damage_type(P::Void, Magic)?;

        self.end()
    }
}
