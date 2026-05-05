use super::*;

impl Generator for PhreakishGusto {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
