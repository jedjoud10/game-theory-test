use owo_colors::{FgColorDisplay, OwoColorize};
use rgb::RGB8;
use tinyrand::{Probability, Rand, StdRand};

use crate::factors::{
    FULLY_STOLEN_POINTS, HALF_STOLEN_POINTS, NOISE, SHARED_POINTS, STOLEN_PENALTY,
};

#[derive(Clone, Copy)]
pub enum Decision {
    Share,
    Steal,
}

impl Decision {
    pub fn color_char(&self, input: char) -> String {
        match self {
            Decision::Share => input.green().to_string(),
            Decision::Steal => input.red().to_string(),
        }
    }

    pub fn noisify(&self, rng: &mut StdRand) -> Decision {
        if rng.next_bool(Probability::new(NOISE)) {
            return match self {
                Decision::Share => Decision::Steal,
                Decision::Steal => Decision::Share,
            }
        } else { *self }
    }
}

pub fn score(a: Decision, b: Decision) -> (i64, i64) {
    match (a, b) {
        (Decision::Share, Decision::Share) => (SHARED_POINTS, SHARED_POINTS),
        (Decision::Share, Decision::Steal) => (STOLEN_PENALTY, FULLY_STOLEN_POINTS),
        (Decision::Steal, Decision::Share) => (FULLY_STOLEN_POINTS, STOLEN_PENALTY),
        (Decision::Steal, Decision::Steal) => (HALF_STOLEN_POINTS, HALF_STOLEN_POINTS),
    }
}
