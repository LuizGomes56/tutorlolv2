use super::*;

impl Generator<Champion> for Lulu {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (1, 0, _3),
                (1, 1, _4),
                (1, 2, _5),
                (1, 3, _6),
            ],
        );
        self.ability(E, [(1, 0, _1)]);
        self.end()
    }
}
