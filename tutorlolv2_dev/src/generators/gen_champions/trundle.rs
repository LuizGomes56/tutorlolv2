use super::*;

impl Generator<Champion> for Trundle {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (0, 0, _1),
                (1, 0, _2),
                (1, 1, _3),
            ],
        );
        self.ability(
            R,
            [
                (0, 0, _1),
                (1, 0, _2),
                (1, 1, _3),
            ],
        );
        self.end()
    }
}
