use super::*;

impl Generator for Irelia {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1)];
        ability![W, (3, 0, _1), (3, 1, _2)];
        ability![E, (2, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
