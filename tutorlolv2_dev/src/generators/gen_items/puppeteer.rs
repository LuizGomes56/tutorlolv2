use super::*;

impl Generator for Puppeteer {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
