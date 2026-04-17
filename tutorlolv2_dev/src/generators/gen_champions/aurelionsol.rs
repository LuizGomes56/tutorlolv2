use super::*;

impl Generator<Champion> for AurelionSol {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(
            Key::Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
                (0, 3, _4),
                (0, 4, _5),
                (0, 5, _6),
                // (1, 0, _7),
            ],
        )
        .ability(Key::W, [(0, 0, _1)])
        .ability(Key::E, [(0, 0, _1), (0, 1, _2)])
        .ability(Key::R, [(0, 0, _1), (0, 0, _2), (1, 0, _3)])
        .progress(Preserve);

        self.end()
    }
}
