use tinyrand::StdRand;

use super::Strategy;
use crate::{
    decision::Decision,
    factors::{ENTITIES_PER_POOL, SHARED_POINTS},
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct TitForTwoTat(u32);
impl Strategy for TitForTwoTat {
    fn decide(&mut self, _round: usize, _rng: &mut StdRand) -> Decision {
        match self.0 >= 2 {
            true => {
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

    fn poolify(&self, _rng: &mut StdRand) -> Box<dyn StratPool> {
        Box::new(vec![TitForTwoTat(0); ENTITIES_PER_POOL])
    }

    fn name(&self) -> &'static str {
        "Tit for Two Tat"
    }
}
