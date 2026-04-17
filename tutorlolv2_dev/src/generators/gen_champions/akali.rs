use super::*;

impl Generator<Champion> for Akali {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.passive(Void, (0, 1), None, None)
            .ability(Key::Q, [(0, 0, Void)])
            .ability(Key::E, [(0, 0, _1Min), (2, 0, _1Max), (2, 1, Max)])
            .ability(Key::R, [(0, 0, _1), (2, 0, _2Min), (2, 1, _2Max)])
            .attr(Area, [Q(Void), R(_1), R(_2Min), R(_2Max)])?
            .attr(Onhit, [P(Void)])?
            .progress(Preserve);

        self.end()
    }
}
