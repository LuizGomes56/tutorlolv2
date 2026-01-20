use super::*;

// #![stable]

impl Generator<Champion> for Ezreal {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, Void)]);
        self.ability(W, [(1, 0, Void)]);
        self.ability(E, [(0, 0, Void)]);
        self.ability(R, [(0, 0, Void), /* (1, 0, Minion) */]);
        self.end()
    }
}
