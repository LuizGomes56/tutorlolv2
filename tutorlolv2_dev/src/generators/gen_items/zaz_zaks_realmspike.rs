use super::*;

impl Generator for ZazZaksRealmspike {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.min(Passive)?.end()
    }
}
