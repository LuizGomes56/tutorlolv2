use super::*;

impl Generator for Yone {
    fn generate(&mut self) -> MayFail {
        let mix = |dmg: &str| f![(dmg * MagicMultiplier) + (dmg * PhysicalMultiplier)];

        self.ability(
            Key::Q,
            [
                (0, Max), /* Critical Strike Damage */
                (1, Min), /* Physical Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .modify(W(Void), mix)?
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .modify(R(Void), mix)?
        .damage_types([W(Void), R(Void)], Mixed)?
        .end()
    }
}
