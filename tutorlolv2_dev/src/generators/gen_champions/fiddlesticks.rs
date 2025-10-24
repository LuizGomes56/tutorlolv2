use super::*;

impl Generator<Champion> for Fiddlesticks {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1), (0, 2, _2), (2, 0, _3), (2, 1, _4)];
        ability![W, (4, 0, _1), (4, 1, _2), (4, 2, _3), (4, 3, _4)];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1), (0, 1, _2)];
    }
}
