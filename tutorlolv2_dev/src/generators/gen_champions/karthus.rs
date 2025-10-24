use super::*;

impl Generator<Champion> for Karthus {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2)];
        ability![E, (2, 0, _1), (2, 1, _2)];
        ability![R, (0, 0, _1)];
    }
}
