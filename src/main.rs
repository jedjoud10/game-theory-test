mod decision;
mod factors;
mod pool;
mod strats;
use factors::ROUNDS;
use strats::*;

use hsv::hsv_to_rgb;
use owo_colors::{OwoColorize, Style};
use std::cmp::Ordering;
use strats::Strategy;
use textplots::{AxisBuilder, Chart, ColorPlot, Plot, Shape, TickDisplayBuilder};
use tinyrand::{Probability, Rand, RandRange, Seeded, StdRand};
use tinyrand_std::ClockSeed;

fn main() {
    let mut pool = Vec::<Box<dyn Strategy>>::default();
    pool.push(Box::new(Random::default()));
    pool.push(Box::new(Nice::default()));
    pool.push(Box::new(NotNice::default()));
    pool.push(Box::new(TitForTat::default()));
    pool.push(Box::new(EachNthStealer::default()));
    pool.push(Box::new(ApologeticGrudge::default()));
    pool.push(Box::new(Grudge::default()));
    pool.push(Box::new(Prober::default()));
    let mut sums = vec![0i64; pool.len()];
    let mut per_round_sums = vec![[0i64; ROUNDS]; pool.len()];

    let mut rng = StdRand::seed(ClockSeed::default().next_u64());
    for (i, s1) in pool.iter().enumerate() {
        for (j, s2) in pool.iter().enumerate() {
            let mut p1 = s1.poolify();
            let mut p2 = s2.poolify();

            let mut temp: [i64; 2] = [0, 0];
            for r in 0..ROUNDS {
                temp[0] = 0;
                temp[1] = 0;
                p1.score(&mut p2, &mut temp, &mut rng, r);
                sums[i] += temp[0];
                sums[j] += temp[1];
                per_round_sums[i][r] = temp[0];
                per_round_sums[j][r] = temp[1];
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
            println!("{line} => ({}, {})", temp[0], temp[1]);
        }
    }

    let mut output = pool.iter().zip(sums.iter()).enumerate().collect::<Vec<_>>();
    output.sort_by(|(_, (_, a)), (_, (_, b))| b.cmp(&a));
    let max = **output.iter().map(|(_, (_, a))| a).max().unwrap() as f32;
    for (i, (strat, &sum)) in output {
        let (r, g, b) = hsv_to_rgb((i as f64 * 360.0) / (pool.len() as f64), 1.0, 0.5);
        println!(
            "{}: {}%",
            strat.name().style(Style::new().truecolor(r, g, b)),
            ((sum as f32 / max) * 100.0)
        );
    }

    let mut c = Chart::new(280, 120, 0.0, (ROUNDS - 1) as f32);

    let mut aca = c.y_tick_display(textplots::TickDisplay::Dense);

    let mut shapes = Vec::<Shape>::default();
    let cpy = &per_round_sums;
    for i in 0..pool.len() {
        shapes.push(Shape::Continuous(Box::new(move |x| {
            if x < 1.0 {
                0.0f32
            } else {
                let last = (x.floor() as usize - 1).min(ROUNDS - 1);
                let cur = (x.floor() as usize).min(ROUNDS - 1);
                let last = cpy[i][last] as f32;
                let cur = cpy[i][cur] as f32;
                let mix = x.fract();
                let val = cur * mix + last * (1.0 - mix);
                val
            }
        })));
    }

    for i in 0..pool.len() {
        let (r, g, b) = hsv_to_rgb((i as f64 * 360.0) / (pool.len() as f64), 1.0, 1.0);
        aca = aca.linecolorplot(&shapes[i], rgb::RGB::new(r, g, b));
    }

    aca.display();
}
