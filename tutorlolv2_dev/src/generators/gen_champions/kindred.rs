use super::*;

impl Generator<Champion> for Kindred {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (3, 0, _1), (3, 1, _2)];
        ability![E, (2, 0, _1), (2, 1, _2)];
    }
}
