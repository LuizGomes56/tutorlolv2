use super::*;

impl Generator for Riven {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (2, 0, _1), (2, 1, _2)];
        ability![W, (0, 0, _1)];
        ability![R, (0, 0, _1), (0, 1, _2)];
    }
}
