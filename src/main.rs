use std::cmp::Ordering;

use owo_colors::OwoColorize;
use textplots::{Chart, Plot, Shape};
use tinyrand::{Probability, Rand, RandRange, Seeded, StdRand};
use tinyrand_std::ClockSeed;

#[derive(Clone, Copy)]
pub enum Decision {
    Share,
    Steal,
}

const SHARED_POINTS: u64 = 4;
const HALF_STOLEN_POINTS: u64 = 1;
const FULLY_STOLEN_POINTS: u64 = 5;
const ENTITIES_PER_POOL: usize = 100;
const ROUNDS: usize = 200;

fn score(a: Decision, b: Decision, rng: &mut StdRand) -> (u64, u64) {
    match (a, b) {
        (Decision::Share, Decision::Share) => (SHARED_POINTS, SHARED_POINTS),
        (Decision::Share, Decision::Steal) => (0, FULLY_STOLEN_POINTS),
        (Decision::Steal, Decision::Share) => (FULLY_STOLEN_POINTS, 0),
        (Decision::Steal, Decision::Steal) => (HALF_STOLEN_POINTS, HALF_STOLEN_POINTS),
    }
}

trait StratPool {
    fn score(&mut self, other: &mut Box<dyn StratPool>, sums: &mut [u64; 2], rng: &mut StdRand, round: usize) {
        let a = self.decide_all(round);
        let b = other.decide_all(round);
        let mut sa = [0; ENTITIES_PER_POOL];
        let mut sb = [0; ENTITIES_PER_POOL];

        for (i, (s1, s2)) in a.iter().zip(b.iter()).enumerate()  {
            (sa[i], sb[i]) = score(*s1, *s2, rng);
            sums[0] += sa[i];
            sums[1] += sb[i];
        }

        self.update_all(sa);
        other.update_all(sb);
    }

    fn decide_all(&mut self, round: usize) -> [Decision; ENTITIES_PER_POOL];
    fn update_all(&mut self, scores: [u64; ENTITIES_PER_POOL]);
}

impl<T: Strategy> StratPool for Vec<T> {
    fn decide_all(&mut self, round: usize) -> [Decision; ENTITIES_PER_POOL] {
        let mut arr = [Decision::Share; ENTITIES_PER_POOL];

        for (i, strat) in self.iter_mut().enumerate() {
            arr[i] = strat.decide(round);
        }

        arr
    }

    fn update_all(&mut self, scores: [u64; ENTITIES_PER_POOL]) {
        for (strat, score) in self.iter_mut().zip(scores) {
            strat.score(score);
        }
    }
}

trait Strategy {
    fn decide(&mut self, round: usize) -> Decision;
    fn poolify(&self) -> Box<dyn StratPool>;
    fn score(&mut self, s: u64) {}
    fn name(&self) -> &'static str; 
}

#[derive(Default)]
struct Random(StdRand);
impl Strategy for Random {
    fn decide(&mut self, round: usize) -> Decision {
        match self.0.next_bool(Probability::new(0.5)) {
            true => Decision::Share,
            false => Decision::Steal,
        }
    }
    
    fn poolify(&self) -> Box<dyn StratPool> {
        let seed = ClockSeed::default().next_u64();
        let vec = (0..ENTITIES_PER_POOL).map(|i| Random(StdRand::seed(i as u64 + seed))).collect::<Vec<_>>();
        Box::new(vec)
    }
    
    fn name(&self) -> &'static str {
        "Random"
    }
}

#[derive(Default, Clone)]
struct TitForTat(bool);
impl Strategy for TitForTat {
    fn decide(&mut self, round: usize) -> Decision {
        let cpy = self.0;
        self.0 = false;
        match cpy {
            true => Decision::Steal,
            false => Decision::Share,
        }
    }

    fn score(&mut self, s: u64) {
        self.0 = s == 0;
    }
    
    fn poolify(&self) -> Box<dyn StratPool> {
        Box::new(vec![TitForTat(false); ENTITIES_PER_POOL])
    }
    
    fn name(&self) -> &'static str {
        "Tit for tat"
    }
}

#[derive(Default, Clone)]
struct Test;
impl Strategy for Test {
    fn decide(&mut self, round: usize) -> Decision {
        match round < (ROUNDS / 2) {
            true => Decision::Steal,
            false => Decision::Share,
        }
    }
    
    fn poolify(&self) -> Box<dyn StratPool> {
        Box::new(vec![TitForTat(false); ENTITIES_PER_POOL])
    }
    
    fn name(&self) -> &'static str {
        "Test"
    }
}

#[derive(Default, Clone)]
struct Nice;
impl Strategy for Nice {
    fn decide(&mut self, round: usize) -> Decision {
        Decision::Share
    }

    fn poolify(&self) -> Box<dyn StratPool> {
        Box::new(vec![Nice; ENTITIES_PER_POOL])
    }
    
    fn name(&self) -> &'static str {
        "Nice"
    }
}

#[derive(Default, Clone)]
struct NotNice;
impl Strategy for NotNice {
    fn decide(&mut self, round: usize) -> Decision {
        Decision::Steal
    }

    fn poolify(&self) -> Box<dyn StratPool> {
        Box::new(vec![NotNice; ENTITIES_PER_POOL])
    }
    
    fn name(&self) -> &'static str {
        "Not Nice"
    }
}


fn main() {
    let mut pool = Vec::<Box<dyn Strategy>>::default();
    pool.push(Box::new(Random::default()));
    pool.push(Box::new(Nice::default()));
    pool.push(Box::new(NotNice::default()));
    pool.push(Box::new(TitForTat::default()));
    pool.push(Box::new(Test::default()));
    let mut sums = vec![0; pool.len()];
    let mut per_round_sums = vec![[0u64;ROUNDS]; pool.len()];

    let mut rng = StdRand::seed(ClockSeed::default().next_u64());
    for (i, s1) in pool.iter().enumerate() {
        for (j, s2) in pool.iter().enumerate() {
            let mut p1 = s1.poolify();
            let mut p2 = s2.poolify();
            
            let mut temp: [u64; 2] = [0, 0];
            for r in 0..ROUNDS {
                p1.score(&mut p2, &mut temp, &mut rng, r);
                sums[i] += temp[0];
                sums[j] += temp[1];
                per_round_sums[i][r] += temp[0];
                per_round_sums[j][r] += temp[1];
            }

            let name1 = s1.name();
            let name2 = s2.name();
            let line = match temp[0].cmp(&temp[1]) {
                Ordering::Less => format!("{} VS {}", name1.red(), name2.green()),
                Ordering::Equal => format!("{} VS {}", name1.yellow(), name2.yellow()),
                Ordering::Greater => format!("{} VS {}", name1.green(), name2.red()),
            };

            let avg = (temp[0] + temp[1]) / 2;
            let d1 = temp[0] as i64 - avg as i64;
            let d2 = temp[1] as i64 - avg as i64;
            println!("{line} => ({}, {}), ({}, {})", temp[0], temp[1], d1, d2);
        }
    }
    
    let mut output = pool.iter().zip(sums.iter()).collect::<Vec<_>>();
    output.sort_by(|(_, a), (_, b)| b.cmp(&a));
    let max = **output.iter().map(|(_, a)| a).max().unwrap() as f32;
    for (strat, &sum) in output {
        println!("{}: {}%", strat.name(), ((sum as f32 / max) * 100.0));
    }

    let mut c = Chart::new(180, 60, 0.0, (ROUNDS-1) as f32);

    let mut aca = &mut c;
    
    let mut shapes = Vec::<Shape>::default();
    let cpy = &per_round_sums;
    for i in 0..pool.len() {
        shapes.push(Shape::Continuous(Box::new(move |x| {
            if x < 0.0 {
                0.0f32
            } else {
                let r = x.round() as usize;
                cpy[i][r] as f32
            }
        })));
    }

    for i in 0..pool.len() {
        aca = aca.lineplot(&shapes[i]);
    }

    aca.display();
}
