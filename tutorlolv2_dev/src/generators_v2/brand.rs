use super::*;

impl Generator for Brand {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (0, 0, _1), (1, 0, _2)];
        ability![E, (1, 0, _1)];
        ability![R, (1, 0, _1), (1, 1, _2)];
    }
}
