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
    pub fn noisify(&self, rng: &mut StdRand) -> Decision {
        if rng.next_bool(Probability::new(NOISE)) {
            return match self {
                Decision::Share => Decision::Steal,
                Decision::Steal => Decision::Share,
            }
        } else { *self }
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            Decision::Share => 1.0f32,
            Decision::Steal => -1.0f32,
        }
    }

    pub fn from_bool(share: bool) -> Decision {
        match share {
            true => Decision::Share,
            false => Decision::Steal,
        }
    }
}

pub fn color_f32_char(val: f32, c: char) -> String {
    let remapped = (val * 0.5 + 0.5).clamp(0.0, 1.0);
    let r = (1.0 - remapped) * 255.0f32;
    let g = remapped * 255.0f32;
    let r = r as u8;
    let g = g as u8;

    c.truecolor(r, g, 0).to_string()
}

pub fn score(a: Decision, b: Decision) -> (i64, i64) {
    match (a, b) {
        (Decision::Share, Decision::Share) => (SHARED_POINTS, SHARED_POINTS),
        (Decision::Share, Decision::Steal) => (STOLEN_PENALTY, FULLY_STOLEN_POINTS),
        (Decision::Steal, Decision::Share) => (FULLY_STOLEN_POINTS, STOLEN_PENALTY),
        (Decision::Steal, Decision::Steal) => (HALF_STOLEN_POINTS, HALF_STOLEN_POINTS),
    }
}
