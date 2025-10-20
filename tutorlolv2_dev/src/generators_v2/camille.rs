use super::*;

impl Generator for Camille {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1), (3, 0, _2)];
        ability![W, (0, 0, _1), (1, 0, _2), (2, 0, _3), (2, 1, _4)];
        ability![E, (0, 0, _1)];
        ability![R, (2, 0, _1)];
    }
}
