use super::*;

impl Generator for Yuumi {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (1, 1, _2), (2, 0, _3)];
        ability![R, (4, 0, _1), (4, 1, _2), (4, 2, _3)];
    }
}
