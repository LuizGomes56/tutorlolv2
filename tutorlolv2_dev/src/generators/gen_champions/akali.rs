use super::*;

// #![stable]

impl Generator<Champion> for Akali {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.passive(Void, (0, 0), None, None);
        self.ability(Q, [(0, 0, Void)]);
        self.ability(E, [(0, 0, _1Min), (2, 0, _1Max), (2, 1, Max)]);
        self.ability(R, [(0, 0, _1), (2, 0, _2Min), (2, 1, _2Max)]);

        self.attr(Attrs::Area, [R::_1, R::_2Min, R::_2Max])?;
        self.attr(Attrs::Area, [Q::Void])?;
        self.attr(Attrs::Onhit, [P::Void])?;
        self.0.end()
    }
}
