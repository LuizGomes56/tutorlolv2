use super::*;

// #![stable]
// #![allow_missing_offsets]

impl Generator<Champion> for Aatrox {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.passive(Void, (0, 0), Some(format!(" * {EnemyBonusHealth}")), None);
        self.ability(
            Key::Q,
            [
                (2, 0, _1Min),
                (2, 1, _1Max),
                (3, 0, _2Min),
                (3, 1, _2Max),
                (5, 0, _3Min),
                (5, 1, _3Max),
            ],
        );
        self.ability(Key::W, [(0, 1, Min), (1, 0, Max)]);

        self.attr(
            Area,
            [Q(_1Min), Q(_1Max), Q(_2Min), Q(_2Max), Q(_3Min), Q(_3Max)],
        )?;

        let default_ability = self.get(Q(_1Min))?;

        let merge =
            |args| self.merge_damage(|[q1, q2, q3]| format!("({q1}) + ({q2}) + ({q3})"), args);

        let q_min = Ability {
            damage: merge([Q(_1Min), Q(_2Min), Q(_3Min)])?,
            ..default_ability.clone()
        };

        let q_max = Ability {
            damage: merge([Q(_1Max), Q(_2Max), Q(_3Max)])?,
            ..default_ability.clone()
        };

        self.insert(Q(Min), q_min);
        self.insert(Q(Max), q_max);
        self.end()
    }
}
