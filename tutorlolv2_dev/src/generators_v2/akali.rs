use super::*;

impl Generator for Akali {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![E, (0, 0, _1), (2, 0, _2), (2, 1, _3)];
        ability![R, (0, 0, _1), (2, 0, _2), (2, 1, _3)];
    }
}
