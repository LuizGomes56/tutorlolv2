use super::*;

impl Generator for Gnar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, Max), (0, 1, Min)])
            .ability_nth(Key::Q, 1, [(0, 0, Mega)])
            .ability(Key::W, [(2, 0, Void)])
            .ability_nth(Key::W, 1, [(0, 0, Mega)])
            .ability(Key::E, [(3, 0, Void)])
            .ability_nth(Key::E, 1, [(0, 0, Mega)])
            .ability(Key::R, [(0, 0, Max), (1, 1, Min)])
            .attr(
                Area,
                [Q(Min), Q(Max), Q(Mega), W(Mega), E(Mega), R(Min), R(Max)],
            )?
            .combo([Ability(Q(Min)), Attack, Attack, Ability(W(Void))])?
            .combo([
                Ability(W(Mega)),
                Attack,
                Ability(Q(Mega)),
                Attack,
                Ability(R(Min)),
                Attack,
                Ability(Q(Mega)),
                Attack,
            ])?
            .progress(Stable)
            .end()
    }
}
