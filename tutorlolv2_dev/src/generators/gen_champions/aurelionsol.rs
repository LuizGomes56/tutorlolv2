use super::*;

impl Generator<Champion> for AurelionSol {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
                (0, 3, _4),
                (0, 4, _5),
                (0, 5, _6),
                (1, 0, _7),
            ],
        );
        self.ability(W, [(0, 0, _1)]);
        self.ability(E, [(0, 0, _1), (0, 1, _2)]);
        self.ability(R, [(0, 0, _1), (0, 0, _2), (1, 0, _3)]);
        self.end()
    }
}
