use super::*;

impl Generator<Champion> for Ashe {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1), (0, 2, _2)];
        ability![W, (0, 1, _1)];
        ability![R, (0, 0, _1)];
    }
}
