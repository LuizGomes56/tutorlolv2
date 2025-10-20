use super::*;

impl Generator for Azir {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (3, 0, _1)];
        ability![E, (1, 0, _1)];
        ability![R, (1, 0, _1)];
    }
}
