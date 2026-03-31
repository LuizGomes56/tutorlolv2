use super::*;

// #![stable]

impl Generator<Champion> for Gnar {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, Max), (0, 1, Min)]);
        self.ability_nth(Key::Q, 1, [(0, 0, Mega)]);
        self.ability(Key::W, [(2, 0, Void)]);
        self.ability_nth(Key::W, 1, [(0, 0, Mega)]);
        self.ability(Key::E, [(3, 0, Void)]);
        self.ability_nth(Key::E, 1, [(0, 0, Mega)]);
        self.ability(Key::R, [(0, 0, Max), (1, 1, Min)]);

        self.attr(
            Area,
            [Q(Min), Q(Max), Q(Mega), W(Mega), E(Mega), R(Min), R(Max)],
        )?;

        self.combo([Ability(Q(Min)), Attack, Attack, Ability(W(Void))])?;

        self.combo([
            Ability(W(Mega)),
            Attack,
            Ability(Q(Mega)),
            Attack,
            Ability(R(Min)),
            Attack,
            Ability(Q(Mega)),
            Attack,
        ])?;

        self.end()
    }
}
