use super::*;

impl Generator for Jhin {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (1, 0, _2), (1, 1, _3)];
        ability![W, (0, 0, _1), (0, 1, _2)];
        ability![E, (1, 0, _1), (1, 1, _2)];
        ability![R, (1, 0, _1), (1, 1, _2), (2, 0, _3), (2, 1, _4)];
    }
}
