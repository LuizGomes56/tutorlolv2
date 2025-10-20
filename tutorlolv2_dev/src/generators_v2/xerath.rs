use super::*;

impl Generator for Xerath {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (2, 0, _1)];
        ability![W, (0, 0, _1), (1, 0, _2)];
        ability![E, (0, 0, _1)];
        ability![R, (1, 0, _1), (2, 0, _2), (2, 1, _3)];
    }
}
