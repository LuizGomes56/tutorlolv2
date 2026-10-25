pub mod ability_levels;
pub mod dragons;
pub mod enemy_stats;
pub mod stats;

#[cfg(feature = "formula-docs")]
pub mod render;

pub trait ReduceApply
where
    Self: Copy + PartialEq + 'static,
    Self::Action: PartialEq + Copy,
{
    type Action;
    fn apply(&mut self, action: Self::Action);
}
