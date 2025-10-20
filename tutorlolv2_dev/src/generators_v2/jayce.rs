use super::*;

impl Generator for Jayce {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 0, _2), (1, 0, _3)];
        ability![W, (0, 0, _1), (0, 1, _2), (0, 0, _3)];
        ability![E, (0, 0, _1), (0, 1, _2)];
    }
}
