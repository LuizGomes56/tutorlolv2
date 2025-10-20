use super::*;

impl Generator for Lissandra {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(W, (0, 0, _1));
        ability!(E, (0, 0, _1));
        ability!(R, (3, 0, _1));
    }
}