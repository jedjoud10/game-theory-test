use crate::{decision::Decision, pool::StratPool};

mod apologetic_grudge;
mod grudge;
mod nice;
mod not_nice;
mod nth_stealer;
mod prober;
mod random;
mod tit_for_tat;

pub use apologetic_grudge::*;
pub use grudge::*;
pub use nice::*;
pub use not_nice::*;
pub use nth_stealer::*;
pub use prober::*;
pub use random::*;
pub use tit_for_tat::*;

pub trait Strategy {
    fn decide(&mut self, round: usize) -> Decision;
    fn poolify(&self) -> Box<dyn StratPool>;
    fn score(&mut self, s: i64) {}
    fn mutate(&mut self) {}
    fn name(&self) -> &'static str;
}
