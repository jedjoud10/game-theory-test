use super::Strategy;
use crate::{
    decision::Decision,
    factors::{ENTITIES_PER_POOL, SHARED_POINTS},
    pool::StratPool,
};

#[derive(Default, Clone)]
pub struct TitForTat(bool);
impl Strategy for TitForTat {
    fn decide(&mut self, round: usize) -> Decision {
        let cpy = self.0;
        self.0 = false;
        match cpy {
            true => Decision::Steal,
            false => Decision::Share,
        }
    }

    fn score(&mut self, s: i64) {
        self.0 = s < SHARED_POINTS;
    }

    fn poolify(&self) -> Box<dyn StratPool> {
        Box::new(vec![TitForTat(false); ENTITIES_PER_POOL])
    }

    fn name(&self) -> &'static str {
        "Tit for tat"
    }
}
