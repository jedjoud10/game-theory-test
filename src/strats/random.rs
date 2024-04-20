use tinyrand::{Probability, Rand, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use super::Strategy;
use crate::{
    decision::Decision,
    factors::ENTITIES_PER_POOL,
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct Random;
impl Strategy for Random {
    fn decide(&mut self, _round: usize, rng: &mut StdRand) -> Decision {
        match rng.next_bool(Probability::new(0.5)) {
            true => Decision::Share,
            false => Decision::Steal,
        }
    }

    fn poolify(&self, _rng: &mut StdRand) -> Box<dyn StratPool> {
        Box::new(vec![Random; ENTITIES_PER_POOL])
    }

    fn name(&self) -> &'static str {
        "Random"
    }
}
