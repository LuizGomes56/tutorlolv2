use super::*;

impl Generator for LichBane {
    fn generate(&mut self) -> MayFail {
        // Spellblade. The other scaling is the attack speed it grants
        self.damage_type(Magic)
            .attr(OnhitMax)
            .min_with(Passive, [0, 2])?
            .end()
    }
}
