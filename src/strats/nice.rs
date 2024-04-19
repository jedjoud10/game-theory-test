use super::Strategy;
use crate::{
    decision::Decision,
    factors::{ENTITIES_PER_POOL, SHARED_POINTS},
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct Nice;
impl Strategy for Nice {
    fn decide(&mut self, round: usize) -> Decision {
        Decision::Share
    }

    fn poolify(&self) -> Box<dyn StratPool> {
        Box::new(vec![Nice; ENTITIES_PER_POOL])
    }

    fn name(&self) -> &'static str {
        "Nice"
    }
}
