use tinyrand::StdRand;

use super::Strategy;
use crate::{
    decision::Decision,
    factors::ENTITIES_PER_POOL,
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct NotNice;
impl Strategy for NotNice {
    fn decide(&mut self, _round: usize, _rng: &mut StdRand) -> Decision {
        Decision::Steal
    }

    fn poolify(&self, _rng: &mut StdRand) -> Box<dyn StratPool> {
        Box::new(vec![NotNice; ENTITIES_PER_POOL])
    }

    fn name(&self) -> &'static str {
        "Not Nice"
    }
}
