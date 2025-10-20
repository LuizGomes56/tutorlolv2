use super::*;

impl Generator for Hecarim {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2)];
        ability![W, (0, 0, _1), (0, 1, _2)];
        ability![E, (3, 0, _1), (3, 1, _2)];
        ability![R, (0, 0, _1)];
    }
}
