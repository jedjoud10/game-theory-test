use owo_colors::OwoColorize;

use crate::strats::Strategy;

// Points given to both parties when they try sharing 
pub const SHARED_POINTS: i64 = 3;

// Points given to both parties when they try stealing
pub const HALF_STOLEN_POINTS: i64 = 1;

// Penalty (should be negative) given to entities whose got stolen from
pub const STOLEN_PENALTY: i64 = 0;

// Points given to the party that "steals" points from an entity whose tried to "share" their points
pub const FULLY_STOLEN_POINTS: i64 = 5;

// Entities per strategy pool
pub const ENTITIES_PER_POOL: usize = 100;

// Noise percentage (0 - 1)
pub const NOISE: f64 = 0.01;

// Total number of rounds that a pool VS pool "fight" should last for
pub const ROUNDS: usize = 200;

// How many entities should be chosen for histogram avg data stuff
pub const HISTOGRAM_ENTITY_COUNT: usize = 20;

pub fn print_params(pool: &[Box<dyn Strategy>]) {
    println!("{}", "Using the following strategies:".underline().italic());
    let all = pool.iter().map(|x| x.name()).collect::<Vec<&str>>().join(", ");
    println!("{all}");
    println!("");
    println!("{}", "With the following parameters:".underline().italic());
    println!("Shared Points: {}", SHARED_POINTS);
    println!("Half Stolen Points: {}", HALF_STOLEN_POINTS);
    println!("Stolen Penalty: {}", STOLEN_PENALTY);
    println!("Fully Stolen Points: {}", FULLY_STOLEN_POINTS);
    println!("Entities Per Pool: {}", ENTITIES_PER_POOL);
    println!("Noise: {}", NOISE);
    println!("Rounds: {}", ROUNDS); 

    println!("");
    println!("{}", "Theoretical Best/Worst case scenario values:".underline().italic());
    println!("Fully taken advantage of: {}", (ENTITIES_PER_POOL as i64) * STOLEN_PENALTY * (ROUNDS as i64));
    println!("Fully took advantage of: {}", (ENTITIES_PER_POOL as i64) * FULLY_STOLEN_POINTS * (ROUNDS as i64));
    println!("Half/half stealing: {}", (ENTITIES_PER_POOL as i64) * HALF_STOLEN_POINTS * (ROUNDS as i64));
    println!("Nice-maxxing: {}", (ENTITIES_PER_POOL as i64) * SHARED_POINTS * (ROUNDS as i64));
    println!("");
}