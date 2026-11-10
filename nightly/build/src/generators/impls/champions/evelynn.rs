use super::*;

impl Generator for Evelynn {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage", Min),
                ("Total Magic Damage", Max),
                ("Bonus Magic Damage", _1),
            ],
        )
        .ability(Key::W, [("Bonus Magic Damage", Void)])
        .ability_nth(1, Key::E, [("Magic Damage", Void)])
        .modify(E(Void), |_| {
            f![60 + 30 * (ELevel - 1) + (0.03 + 0.00015 * AbilityPower) * EnemyMaxHealth]
        })?
        .ability(Key::E, [("Empowered Magic Damage", _1)])
        .modify(E(_1), |_| {
            f![80 + 40 * (ELevel - 1) + (0.04 + 0.00025 * AbilityPower) * EnemyMaxHealth]
        })?
        .ability(
            Key::R,
            [
                ("Magic Damage", Min),
                ("Empowered Damage", Max),
            ],
        )
        .combos([
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(R(Max)),
            ],
            &[Ability(Q(Max)), Ability(E(Void)), Ability(R(Max))],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(W(Void)),
                Ability(R(Max)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(R(Max)),
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Ability(R(Max)),
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
            ],
        ])?
        .end()
    }
}
