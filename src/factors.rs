// Points given to both parties when they try sharing 
pub const SHARED_POINTS: i64 = 3;

// Points given to both parties when they try stealing
pub const HALF_STOLEN_POINTS: i64 = 1;

// Penalty (should be negative) given to entities whose got stolen from
pub const STOLEN_PENALTY: i64 = -1;

// Points given to the party that "steals" points from an entity whose tried to "share" their points
pub const FULLY_STOLEN_POINTS: i64 = 3;

// Entities per strategy pool
pub const ENTITIES_PER_POOL: usize = 100;

// Noise percentage (0 - 1)
pub const NOISE: f64 = 0.01;

// Total number of rounds that a pool VS pool "fight" should last for
pub const ROUNDS: usize = 100;
