use super::*;

impl Generator for RekSai {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2), (0, 0, _3)];
        ability![W, (0, 0, _1)];
        ability![E, (0, 0, _1), (1, 0, _2)];
        ability![R, (0, 0, _1)];
    }
}
