use super::*;

impl Generator for Yone {
    fn generate(&mut self) -> MayFail {
        let mix = |dmg: &str| f![((dmg) * MagicMultiplier) + ((dmg) * PhysicalMultiplier)];

        self.ability(
            Key::Q,
            [
                ("Critical Strike Damage", Max),
                ("Physical Damage", Min),
            ],
        )
        .ability(Key::W, [("Magic Damage", Void)])
        .modify(W(Void), mix)?
        .ability(Key::R, [("Magic Damage", Void)])
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
