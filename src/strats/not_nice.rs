use super::Strategy;
use crate::{
    decision::Decision,
    factors::{ENTITIES_PER_POOL, SHARED_POINTS},
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct NotNice;
impl Strategy for NotNice {
    fn decide(&mut self, round: usize) -> Decision {
        Decision::Steal
    }

    fn poolify(&self) -> Box<dyn StratPool> {
        Box::new(vec![NotNice; ENTITIES_PER_POOL])
    }

    fn name(&self) -> &'static str {
        "Not Nice"
    }
}
