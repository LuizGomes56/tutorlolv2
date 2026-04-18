use super::*;

impl Generator for KircheisShard {
    fn generate(&mut self) -> MayFail {
        self.const_min_dmg(50).attr(OnhitMax).damage_type(Magic).end()
    }
}
