use super::*;

// #![stable]

impl Generator<Champion> for Ashe {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 1, Min), (0, 2, Max)]);
        self.ability(Key::W, [(0, 1, Void)]);
        self.ability(Key::R, [(0, 0, Void)]);

        self.attr(Area, [R(Void), W(Void)])?;
        self.combo([Attack, Ability(W(Void)), Ability(R(Void)), Attack])?;

        self.end()
    }
}
