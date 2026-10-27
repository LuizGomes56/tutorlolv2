use super::*;

impl Generator for Yone {
    fn generate(&mut self) -> MayFail {
        let mix = |dmg: &str| f![((dmg) * MagicMultiplier) + ((dmg) * PhysicalMultiplier)];

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
        .combos([
            &[
                Ability(R(Void)),
                Ability(Q(Min)),
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
                Ability(Q(Min)),
            ],
            &[
                Ability(R(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
                Attack,
            ],
        ])?
        .end()
    }
}
