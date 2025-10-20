use super::*;

impl Generator for Annie {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (0, 0, _1)];
        ability![E, (1, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
