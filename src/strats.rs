use crate::{decision::Decision, pool::StratPool};

mod twice_grudge;
mod grudge;
mod nice;
mod not_nice;
mod nth_stealer;
mod prober;
mod random;
mod tit_for_tat;

pub use twice_grudge::*;
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
    fn score(&mut self, _s: i64) {}
    fn name(&self) -> &'static str;
}
