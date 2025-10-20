use super::*;

impl Generator for Darius {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2)];
        ability![W, (0, 0, _1)];
        ability![R, (0, 0, _1), (0, 1, _2), (0, 2, _3)];
    }
}
