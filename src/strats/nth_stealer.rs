use tinyrand::{Rand, RandRange, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use super::Strategy;
use crate::{
    decision::Decision,
    factors::{ENTITIES_PER_POOL, SHARED_POINTS},
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct EachNthStealer(usize);
impl Strategy for EachNthStealer {
    fn decide(&mut self, round: usize) -> Decision {
        self.0 += 1;
        self.0 %= 10;
        match self.0 == 0 {
            true => Decision::Steal,
            false => Decision::Share,
        }
    }

    fn poolify(&self) -> Box<dyn StratPool> {
        let seed = ClockSeed::default().next_u64();
        let mut rng = StdRand::seed(seed);
        let vec = (0..ENTITIES_PER_POOL)
            .map(|i| EachNthStealer(rng.next_range(0..9)))
            .collect::<Vec<_>>();
        Box::new(vec)
    }

    fn name(&self) -> &'static str {
        "Each N-th Stealer"
    }
}
