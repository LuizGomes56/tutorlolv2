use super::*;

impl Generator for Taliyah {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Min))?
            .map_change(Q(Min), "Initial rock base damage")?
            .map_change(Q(Min), "Initial rock AP ratio")?
            .derive(Q(_1), Q(Min), |q| q.scale(1.8))?
            .comment(Q(_1), "Empowered Damage")
            .derive(Q(_2), Q(Min), |q| q.scale(0.4))?
            .comment(Q(_2), "Reduced Damage")
            .derive(Q(Max), Q(Min), |q| q.scale(2.6))?
            .comment(Q(Max), "Total Magic Damage")
            .take("Magic Damage", E(Void))?
            .take("Detonation Magic Damage", E(_1))?
            .take("Total Maximum Detonation Damage", E(_2))?
            .end()
    }
}
