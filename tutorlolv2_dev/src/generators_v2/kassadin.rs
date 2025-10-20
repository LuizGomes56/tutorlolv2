use super::*;

impl Generator for Kassadin {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (0, 0, _1)];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1), (1, 0, _2), (1, 1, _3), (1, 2, _4)];
    }
}
