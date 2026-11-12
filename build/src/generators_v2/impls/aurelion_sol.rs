use super::*;

impl GeneratorV2 for AurelionSol {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage per Second", Q(_1))?
            .take("Bonus Magic Damage", Q(_2))?
            // (3.1% Stardust)% of maximum health, inside the icon of Cosmic Creator, that the
            // model can't read: 100 Stardust is 3.1%
            .edit(Q(_2), |q| f![q + 0.031 * Stacks / 100 * EnemyMaxHealth])
            .take("Magic Damage per Tick", Q(Min))?
            .take("Total Maximum Magic Damage", Q(Max))?
            .take("Magic Damage per Tick", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take_in("Falling Star", "Magic Damage", R(Void))?
            .take_in("The Skies Descend", "Magic Damage", R(_1))?
            .take("Empowered Magic Damage", R(_2))?
            .comment(R(_1), "The Skies Descend")
            .comment(R(_2), "The Skies Descend empowered by Stardust")
            .combos([
                &[Ability(E(Max)), Ability(R(Void)), Ability(Q(Max))],
                &[
                    Ability(Q(Max)),
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                ],
                &[Ability(E(Max)), Ability(R(Void))],
                &[Ability(E(Max)), Ability(Q(Max))],
            ])?
            .end()
    }
}
