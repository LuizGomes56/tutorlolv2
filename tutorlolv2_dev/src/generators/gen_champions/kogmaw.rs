use super::*;

impl Generator<Champion> for KogMaw {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (0, 1, _1)];
        ability![E, (1, 0, _1)];
        ability![R, (0, 0, _1), (0, 1, _2)];
    }
}
