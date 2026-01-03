use super::*;

// #![stable]

impl Generator<Champion> for Amumu {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, Void)]);
        self.ability(W, [(0, 0, Void)]);
        self.ability(E, [(0, 0, Void)]);
        self.ability(R, [(0, 0, Void)]);
        self.end()
    }
}
