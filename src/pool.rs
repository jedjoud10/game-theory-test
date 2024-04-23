use crate::{
    decision::{score, Decision},
    factors::ENTITIES_PER_POOL,
    strats::Strategy,
};
use tinyrand::StdRand;

pub fn score_pool(
    this: &mut Box<dyn StratPool>,
    other: &mut Box<dyn StratPool>,
    sums: &mut [i64; 2],
    decisions: &mut [f32; 2],
    entity_count_decision_avg: usize,
    noise_rng: &mut StdRand,
    pool_rng: &mut StdRand,
    round: usize,
) {
    let a = this.decide_all(round, pool_rng);
    let b = other.decide_all(round, pool_rng);
    let mut sa = [0; ENTITIES_PER_POOL];
    let mut sb = [0; ENTITIES_PER_POOL];

    for (i, (&s1, &s2)) in a.iter().zip(b.iter()).enumerate() {
        let s1 = s1.noisify(noise_rng);
        let s2 = s2.noisify(noise_rng);
        
        (sa[i], sb[i]) = score(s1, s2);

        if i < entity_count_decision_avg {
            decisions[0] += s1.to_f32();
            decisions[1] += s2.to_f32();
        }

        sums[0] += sa[i];
        sums[1] += sb[i];
    }

    this.update_all(sa);
    other.update_all(sb);
}

pub trait StratPool {
    fn decide_all(&mut self, round: usize, rng: &mut StdRand) -> [Decision; ENTITIES_PER_POOL];
    fn update_all(&mut self, scores: [i64; ENTITIES_PER_POOL]);
}

impl<T: Strategy> StratPool for Vec<T> {
    fn decide_all(&mut self, round: usize, rng: &mut StdRand) -> [Decision; ENTITIES_PER_POOL] {
        let mut arr = [Decision::Share; ENTITIES_PER_POOL];

        for (i, strat) in self.iter_mut().enumerate() {
            arr[i] = strat.decide(round, rng);
        }

        arr
    }

    fn update_all(&mut self, scores: [i64; ENTITIES_PER_POOL]) {
        for (strat, score) in self.iter_mut().zip(scores) {
            strat.score(score);
        }
    }
}
