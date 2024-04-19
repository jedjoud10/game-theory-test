use tinyrand::{Probability, Rand, StdRand};

use crate::factors::{
    FULLY_STOLEN_POINTS, HALF_STOLEN_POINTS, NOISE, SHARED_POINTS, STOLEN_PENALTY,
};

#[derive(Clone, Copy)]
pub enum Decision {
    Share,
    Steal,
}

pub fn score(mut a: Decision, mut b: Decision, rng: &mut StdRand) -> (i64, i64) {
    if rng.next_bool(Probability::new(NOISE)) {
        a = match a {
            Decision::Share => Decision::Steal,
            Decision::Steal => Decision::Share,
        }
    }

    if rng.next_bool(Probability::new(NOISE)) {
        b = match b {
            Decision::Share => Decision::Steal,
            Decision::Steal => Decision::Share,
        }
    }

    match (a, b) {
        (Decision::Share, Decision::Share) => (SHARED_POINTS, SHARED_POINTS),
        (Decision::Share, Decision::Steal) => (STOLEN_PENALTY, FULLY_STOLEN_POINTS),
        (Decision::Steal, Decision::Share) => (FULLY_STOLEN_POINTS, STOLEN_PENALTY),
        (Decision::Steal, Decision::Steal) => (HALF_STOLEN_POINTS, HALF_STOLEN_POINTS),
    }
}
