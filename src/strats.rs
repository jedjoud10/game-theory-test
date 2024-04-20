use crate::{decision::Decision, pool::StratPool};

mod tit_for_two_tat;
mod grudge;
mod nice;
mod not_nice;
mod nth_stealer;
mod prober;
mod random;
mod tit_for_tat;

use tinyrand::StdRand;
pub use tit_for_two_tat::*;
pub use grudge::*;
pub use nice::*;
pub use not_nice::*;
pub use nth_stealer::*;
pub use prober::*;
pub use random::*;
pub use tit_for_tat::*;

pub trait Strategy {
    fn decide(&mut self, round: usize, rng: &mut StdRand) -> Decision;
    fn poolify(&self, rng: &mut StdRand) -> Box<dyn StratPool>;
    fn score(&mut self, _s: i64) {}
    fn name(&self) -> &'static str;
}
