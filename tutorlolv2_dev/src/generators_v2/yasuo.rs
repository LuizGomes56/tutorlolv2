use super::*;

impl Generator for Yasuo {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![E, (0, 0, _1), (2, 0, _2), (2, 1, _3), (2, 2, _4)];
        ability![R, (3, 0, _1)];
    }
}
