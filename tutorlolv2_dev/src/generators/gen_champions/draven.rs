use super::*;

impl Generator<Champion> for Draven {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(E, [(0, 0, _1)]);
        self.ability(
            R,
            [
                (0, 0, _1),
                (0, 1, _2),
                (4, 0, _3),
                (4, 1, _4),
            ],
        );
        self.end()
    }
}
