use super::*;

impl Generator<Champion> for Nami {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (1, 1, _1), (1, 2, _2)];
        ability![E, (0, 0, _1), (0, 2, _2)];
        ability![R, (0, 0, _1)];
    }
}
