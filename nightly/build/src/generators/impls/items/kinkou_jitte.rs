use super::*;

impl Generator for KinkouJitte {
    fn generate(&mut self) -> MayFail {
        // Between the Ribs: consuming a Weakpoint. The other scaling is the
        // movement speed it grants
        self.damage_type(True).min_with(Passive, [0])?.end()
    }
}
