use super::*;

impl Generator for NimbusCloak {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Passive */
            .end()
    }
}
