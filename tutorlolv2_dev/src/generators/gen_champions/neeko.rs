use super::*;

// #![stable]

impl Generator<Champion> for Neeko {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (2, 0, Min), (2, 1, Max)]);
        self.ability(Key::W, [(1, 0, Void)]);
        self.ability(Key::E, [(0, 0, Void)]);
        self.ability(Key::R, [(2, 0, Void)]);
        self.attr(Area, [Q(_1), Q(Min), Q(Max), E(Void), R(Void)])?;
        self.end()
    }
}
