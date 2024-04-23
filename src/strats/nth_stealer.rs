use tinyrand::{Rand, RandRange, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use super::Strategy;
use crate::{
    decision::Decision,
    factors::ENTITIES_PER_POOL,
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct EachNthStealer(usize);
impl Strategy for EachNthStealer {
    fn decide(&mut self, _round: usize, _rng: &mut StdRand) -> Decision {
        self.0 += 1;
        self.0 %= 10;
        Decision::from_bool(self.0 != 0)
    }

    fn poolify(&self, _rng: &mut StdRand) -> Box<dyn StratPool> {
        let seed = ClockSeed.next_u64();
        let mut rng = StdRand::seed(seed);
        let vec = (0..ENTITIES_PER_POOL)
            .map(|_i| EachNthStealer(rng.next_range(0..2)))
            .collect::<Vec<_>>();
        Box::new(vec)
    }

    fn name(&self) -> &'static str {
        "Each N-th Stealer"
    }
}
