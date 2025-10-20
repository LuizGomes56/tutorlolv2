use super::*;

impl Generator for Velkoz {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (0, 0, _1), (1, 0, _2), (1, 1, _3)];
        ability![E, (0, 0, _1)];
        ability![R, (3, 0, _1), (3, 1, _2)];
    }
}
