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
}
impl Strategy for Prober {
    fn decide(&mut self, _round: usize) -> Decision {
        if !self.tested {
            self.tested = true;
            return Decision::Steal;
        }

        match self.abuse {
            true => Decision::Steal,
            false => Decision::Share,
        }
    }

    fn score(&mut self, s: i64) {
        if self.tested && s == FULLY_STOLEN_POINTS {
            self.abuse = true;
        }
    }

    fn poolify(&self) -> Box<dyn StratPool> {
        Box::new(vec![
            Prober {
                tested: false,
                abuse: false
            };
            ENTITIES_PER_POOL
        ])
    }

    fn name(&self) -> &'static str {
        "Prober"
    }
}
