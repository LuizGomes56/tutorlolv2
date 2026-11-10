use super::*;

impl Generator for Naafiri {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Ability {
                name: "Packmate".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Pet: Packmate basic attack".into(),
                damage: f![10 + 10 / 17 * (Level - 1) + 0.04 * BonusAd],
            },
        )
        .ability(
            Key::Q,
            [
                ("Minimum Total Physical Damage", Min),
                ("Maximum Total Physical Damage", Max),
                ("Total Bleed Physical Damage", _1),
            ],
        )
        .ability(
            Key::E,
            [
                ("Dash Physical Damage", Min),
                ("Total Physical Damage", Max),
            ],
        )
        .ability(
            Key::R,
            [
                ("Physical Damage", Void),
                ("Physical Damage per  Packmate", _1),
            ],
        )
        .combos([
            &[
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Attack,
                Ability(Q(Min)),
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
                Ability(E(Max)),
                Attack,
            ],
        ])?
        .end()
    }
}
