use super::*;

impl Generator<Champion> for Taliyah {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
                (3, 0, _4),
            ],
        );
        self.ability(
            E,
            [
                (0, 0, _1),
                (1, 0, _2),
                (2, 0, _3),
            ],
        );
        self.end()
    }
}
