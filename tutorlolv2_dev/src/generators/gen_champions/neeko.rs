use super::*;

impl Generator for Neeko {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (2, 0, Min), (2, 1, Max)])
            .ability(Key::W, [(1, 0, Void)])
            .ability(Key::E, [(0, 0, Void)])
            .ability(Key::R, [(2, 0, Void)])
            .attr(Area, [Q(_1), Q(Min), Q(Max), E(Void), R(Void)])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(W(Void)),
                Ability(R(Void)),
            ])?
            .combo([Ability(Q(_1)), Attack, Ability(W(Void))])?
            .progress(Stable)
            .end()
    }
}
