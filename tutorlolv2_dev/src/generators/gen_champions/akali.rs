use super::*;

impl Generator for Akali {
    fn generate(&mut self) -> MayFail {
        self.passive(Void, (0, 1), None, None)
            .ability(Key::Q, [(0, 0, Void)])
            .ability(Key::E, [(0, 0, _1Min), (2, 0, _1Max), (2, 1, Max)])
            .ability(Key::R, [(0, 0, _1), (2, 0, _2Min), (2, 1, _2Max)])
            .attr(Area, [Q(Void), R(_1), R(_2Min), R(_2Max)])?
            .attr(Onhit, [P(Void)])?
            .combo([
                Ability(Q(Void)),
                Ability(E(_1Min)),
                Attack,
                Ability(P(Void)),
                Ability(E(_1Max)),
                Ability(R(_1)),
            ])?
            .progress(Stable)
            .end()
    }
}
