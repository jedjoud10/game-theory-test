use tinyrand::{Probability, Rand, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use super::Strategy;
use crate::{
    decision::Decision,
    factors::{ENTITIES_PER_POOL, SHARED_POINTS},
    pool::StratPool,
};

#[derive(Default)]
pub struct Random(StdRand);
impl Strategy for Random {
    fn decide(&mut self, round: usize) -> Decision {
        match self.0.next_bool(Probability::new(0.5)) {
            true => Decision::Share,
            false => Decision::Steal,
        }
    }

    fn poolify(&self) -> Box<dyn StratPool> {
        let seed = ClockSeed::default().next_u64();
        let vec = (0..ENTITIES_PER_POOL)
            .map(|i| Random(StdRand::seed(i as u64 + seed)))
            .collect::<Vec<_>>();
        Box::new(vec)
    }

    fn name(&self) -> &'static str {
        "Random"
    }
}
