use super::*;

impl Generator for Stridebreaker {
    fn generate(&mut self) -> MayFail {
        // Breaking Shockwave. The other scaling is the movement speed it grants
        self.damage_type(Physical).min_with(Active, [0])?.end()
    }
}
