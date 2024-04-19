use super::Strategy;
use crate::{
    decision::Decision,
    factors::{ENTITIES_PER_POOL, SHARED_POINTS},
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct ApologeticGrudge(u32);
impl Strategy for ApologeticGrudge {
    fn decide(&mut self, round: usize) -> Decision {
        match self.0 > 2 {
            true => {
                self.0 = 0;
                Decision::Steal
            }
            false => Decision::Share,
        }
    }

    fn score(&mut self, s: i64) {
        if s < SHARED_POINTS {
            self.0 += 1;
        } else {
            self.0 = 0;
        }
    }

    fn poolify(&self) -> Box<dyn StratPool> {
        Box::new(vec![ApologeticGrudge(0); ENTITIES_PER_POOL])
    }

    fn name(&self) -> &'static str {
        "Apologetic Grudge"
    }
}
