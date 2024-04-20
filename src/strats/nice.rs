use tinyrand::StdRand;

use super::Strategy;
use crate::{
    decision::Decision,
    factors::ENTITIES_PER_POOL,
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct Nice;
impl Strategy for Nice {
    fn decide(&mut self, _round: usize, _rng: &mut StdRand) -> Decision {
        Decision::Share
    }

    fn poolify(&self, _rng: &mut StdRand) -> Box<dyn StratPool> {
        Box::new(vec![Nice; ENTITIES_PER_POOL])
    }

    fn name(&self) -> &'static str {
        "Nice"
    }
}
