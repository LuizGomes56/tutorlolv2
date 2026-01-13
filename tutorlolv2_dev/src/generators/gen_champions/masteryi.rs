use super::*;

impl Generator<Champion> for MasterYi {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (3, 0, _1),
                (3, 1, _2),
                (3, 2, _3),
                (3, 3, _4),
                (3, 4, _5),
                (3, 5, _6),
                (3, 6, _7),
            ],
        );
        self.ability(
            W,
            [(2, 0, _1), (2, 1, _2)],
        );
        self.ability(E, [(0, 0, _1)]);
        self.end()
    }
}
