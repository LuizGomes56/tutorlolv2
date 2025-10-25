use super::*;

impl Generator<Champion> for Kennen {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (0, 0, _1), (2, 0, _2)];
        ability![E, (0, 0, _1), (0, 1, _2)];
        ability![R, (2, 0, _1), (3, 0, _2)];
    }
}
