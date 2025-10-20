use super::*;

impl Generator for Kayle {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![E, (0, 0, _1), (2, 0, _2)];
        ability![R, (1, 0, _1)];
    }
}
