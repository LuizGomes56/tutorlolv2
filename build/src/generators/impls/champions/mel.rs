use super::*;

impl Generator for Mel {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Initial Explosion Magic Damage", Min),
                ("Total Magic Damage", Max),
                ("Magic Damage per Subsequent Explosion", _1),
            ],
        )
        .ability(
            Key::E,
            [
                ("Orb Magic Damage", Void),
                ("Field Magic Damage per Second", _1),
            ],
        )
        .ability(Key::R, [("Magic Damage", Void)])
        .modify(R(Void), |_| {
            f![125
                + 37.5 * (RLevel - 1)
                + 0.3 * AbilityPower
                + Stacks * (4 + 1.5 * (RLevel - 1) + 0.04 * AbilityPower)]
        })?
        .comment(
            R(Void),
            "Magic damage, Stacks = Overwhelm stacks on the target",
        )?
        .combos([
            &[
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[Ability(E(Void)), Ability(Q(Max)), Ability(R(Void))],
            &[
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
        ])?
        .end()
    }
}
