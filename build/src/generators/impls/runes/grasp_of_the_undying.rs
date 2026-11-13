use super::*;

impl Generator for GraspOfTheUndying {
    fn generate(&mut self) -> MayFail {
        let [melee, ranged] = self.description(0).map(RegExtractor::capture_numbers)?[4..6] else {
            return Err("No numbers [melee, ranged] at self[0, 4..6]".into());
        };

        self.assign(Melee, Min, f![melee * MaxHealth])
            .assign(Ranged, Min, f![ranged * MaxHealth])
            .damage_type(Magic)
            .end()
    }
}
