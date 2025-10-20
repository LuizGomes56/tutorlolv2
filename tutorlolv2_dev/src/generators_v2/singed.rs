use super::*;

impl Generator for Singed {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (1, 0, _1), (1, 1, _2), (1, 2, _3));
        ability!(E, (0, 0, _1));
    }
}