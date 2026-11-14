use super::*;

impl Generator for WardensMail {
    fn generate(&mut self) -> MayFail {
        // Rock Solid
        let [rock_solid_flat_reduction, rock_solid_max_reduction] =
            self.description(Named("Rock Solid"))?.capture(regex!(
                r"by (\d+), with a '''maximum''' of (\d+)% reduction"
            ))?;

        self.constant("ROCK_SOLID_FLAT_REDUCTION", rock_solid_flat_reduction)?
            .constant("ROCK_SOLID_MAX_REDUCTION", rock_solid_max_reduction / 100.0)?;

        Ok(())
    }
}
