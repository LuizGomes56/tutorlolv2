use super::*;

impl Generator<Champion> for Vi {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1), (1, 1, _2)];
        ability![W, (1, 0, _1)];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
