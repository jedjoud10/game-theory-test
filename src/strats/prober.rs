use tinyrand::StdRand;

use super::Strategy;
use crate::{
    decision::Decision,
    factors::{ENTITIES_PER_POOL, FULLY_STOLEN_POINTS},
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct Prober {
    tested: bool,
    abuse: bool,
    oops: bool,
}
impl Strategy for Prober {
    fn decide(&mut self, _round: usize, _rng: &mut StdRand) -> Decision {
        if !self.tested {
            self.tested = true;
            return Decision::Steal;
        }

        Decision::from_bool(!self.abuse)
    }

    fn score(&mut self, s: i64) {
        if self.tested && s == FULLY_STOLEN_POINTS {
            self.abuse = true;
        }

        if s != FULLY_STOLEN_POINTS {
            if self.oops {
                self.abuse = false;
                self.oops = false;
                self.tested = true;
            }

            self.oops = true;
        }
    }

    fn poolify(&self, _rng: &mut StdRand) -> Box<dyn StratPool> {
        Box::new(vec![
            Prober {
                tested: false,
                abuse: false,
                oops: false,
            };
            ENTITIES_PER_POOL
        ])
    }

    fn name(&self) -> &'static str {
        "Prober"
    }
}
