use super::*;

impl Generator for Anivia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Magic Damage */
                (2, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Max), /* Enhanced Damage */
                (1, Min), /* Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, Max), /* Empowered Damage per Tick */
                (2, Min), /* Magic Damage per Tick */
            ],
        )
        .combo([
            Ability(Q(Min)),
            Ability(Q(Min)),
            Ability(E(Min)),
            Ability(R(Max)),
            Attack,
            Attack,
            Ability(E(Min)),
            Attack,
            Ability(R(Max)),
        ])?
        .combo([
            Ability(R(Max)),
            Ability(Q(Min)),
            Attack,
            Ability(Q(Min)),
            Ability(E(Max)),
            Attack,
            Ability(R(Max)),
        ])?
        .combo([
            Ability(Q(Min)),
            Ability(R(Max)),
            Attack,
            Ability(Q(Min)),
            Ability(E(Max)),
            Attack,
            Ability(R(Max)),
        ])?
        .combo([
            Ability(R(Max)),
            Attack,
            Ability(E(Max)),
            Ability(Q(Min)),
            Ability(Q(Min)),
            Attack,
        ])?
        .combo([
            Ability(Q(Min)),
            Ability(Q(Min)),
            Ability(E(Max)),
            Ability(R(Max)),
        ])?
        .combo([Ability(Q(Min)), Ability(E(Max)), Ability(Q(Min))])?
        .combo([
            Ability(R(Max)),
            Attack,
            Ability(E(Max)),
            Ability(R(Max)),
            Attack,
        ])?
        .combo([
            Ability(Q(Min)),
            Attack,
            Ability(Q(Min)),
            Ability(E(Max)),
            Attack,
        ])?
        .end()
    }
}
