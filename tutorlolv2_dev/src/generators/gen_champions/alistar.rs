use super::*;

// #![stable]

impl Generator<Champion> for Alistar {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, Void)]);
        self.ability(W, [(0, 0, Void)]);
        self.ability(E, [(0, 0, Min), (0, 1, Max)]);
        self.end()
    }
}
