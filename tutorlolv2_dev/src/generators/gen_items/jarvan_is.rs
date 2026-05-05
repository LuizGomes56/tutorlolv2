use super::*;

impl Generator for JarvanIs {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
