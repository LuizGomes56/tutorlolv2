use super::*;

impl Generator for Nocturne {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (1, 0, _2));
        ability!(E, (1, 0, _1), (1, 1, _2));
        ability!(R, (1, 0, _1));
    }
}