use super::*;

impl Generator<Champion> for TwistedFate {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(
            W,
            [
                (1, 0, _1),
                (2, 0, _2),
                (5, 0, _3),
            ],
        );
        self.ability(E, [(0, 1, _1)]);
        self.end()
    }
}
