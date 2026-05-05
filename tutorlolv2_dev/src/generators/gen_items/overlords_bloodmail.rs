use super::*;

impl Generator for OverlordsBloodmail {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
