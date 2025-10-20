use super::*;

impl Generator for Zoe {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2)];
        ability![W, (1, 0, _1), (1, 1, _2)];
        ability![E, (1, 0, _1), (2, 0, _2), (2, 1, _3)];
    }
}
