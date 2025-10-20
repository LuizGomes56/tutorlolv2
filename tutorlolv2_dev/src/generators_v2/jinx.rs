use super::*;

impl Generator for Jinx {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![W, (0, 0, _1)];
        ability![E, (0, 0, _1)];
        ability![R, (1, 0, _1), (1, 1, _2), (2, 0, _3), (2, 1, _4)];
    }
}
