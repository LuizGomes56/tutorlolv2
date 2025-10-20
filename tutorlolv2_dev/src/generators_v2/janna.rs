use super::*;

impl Generator for Janna {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2), (0, 2, _3)];
        ability![W, (0, 0, _1)];
        ability![E, (2, 0, _1)];
    }
}
