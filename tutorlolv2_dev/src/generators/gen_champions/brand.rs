use super::*;

impl Generator<Champion> for Brand {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (0, 0, _1), (1, 0, _2)];
        ability![E, (1, 0, _1)];
        ability![R, (1, 0, _1), (1, 1, _2)];
    }
}
