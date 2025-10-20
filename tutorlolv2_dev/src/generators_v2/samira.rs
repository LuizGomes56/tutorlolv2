use super::*;

impl Generator for Samira {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (2, 0, _1), (2, 1, _2)];
        ability![E, (0, 1, _1)];
        ability![R, (1, 0, _1), (1, 1, _2), (1, 2, _3), (1, 3, _4)];
    }
}
