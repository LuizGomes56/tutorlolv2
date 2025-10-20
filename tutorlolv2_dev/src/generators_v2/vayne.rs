use super::*;

impl Generator for Vayne {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (1, 0, _1), (2, 0, _2), (2, 1, _3)];
        ability![E, (0, 0, _1), (1, 0, _2), (1, 1, _3)];
        ability![R, (0, 0, _1)];
    }
}
