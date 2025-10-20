use super::*;

impl Generator for Sivir {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (1, 0, _2), (1, 1, _3)];
        ability![W, (0, 1, _1), (0, 2, _2), (0, 3, _3), (0, 4, _4)];
    }
}
