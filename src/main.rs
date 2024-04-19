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
use textplots::{Chart, ColorPlot, Shape, TickDisplayBuilder};
use tinyrand::{Rand, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use crate::pool::score_pool;

fn main() {
    // Create a pool of multiple boxed strategies so we can duplicate them into their own entity pools
    let mut pool = Vec::<Box<dyn Strategy>>::default();
    pool.push(Box::<Random>::default());
    pool.push(Box::new(Nice));
    pool.push(Box::new(NotNice));
    pool.push(Box::<TitForTat>::default());
    pool.push(Box::<EachNthStealer>::default());
    pool.push(Box::<ApologeticGrudge>::default());
    pool.push(Box::<Grudge>::default());
    pool.push(Box::<Prober>::default());

    // Total strategy point sum and point sums gained each round 
    let mut total_sums = vec![0i64; pool.len()];
    let mut delta_sums = vec![[0i64; ROUNDS]; pool.len()];

    let mut rng = StdRand::seed(ClockSeed.next_u64());
    for (i, s1) in pool.iter().enumerate() {
        for (j, s2) in pool.iter().enumerate() {
            let mut p1 = s1.poolify();
            let mut p2 = s2.poolify();

            // Make the 2 pools "fight" each other for n number of rounds
            let mut temp: [i64; 2] = [0, 0];
            for r in 0..ROUNDS {
                temp[0] = 0;
                temp[1] = 0;
                score_pool(&mut p1, &mut p2, &mut temp, &mut rng, r);
                total_sums[i] += temp[0];
                total_sums[j] += temp[1];
                delta_sums[i][r] = temp[0];
                delta_sums[j][r] = temp[1];
            }

            // Some cool debugging to see which strategy worked best in this special case
            let name1 = s1.name();
            let name2 = s2.name();
            let line = match temp[0].cmp(&temp[1]) {
                Ordering::Less => format!("{} VS {}", name1.red(), name2.green()),
                Ordering::Equal => format!("{} VS {}", name1.yellow(), name2.yellow()),
                Ordering::Greater => format!("{} VS {}", name1.green(), name2.red()),
            };

            println!("{line} => ({}, {})", temp[0], temp[1]);
        }
    }

    // Very ugly code to try to find relative percentage and to sort the strats
    let mut output = pool.iter().zip(total_sums.iter()).enumerate().collect::<Vec<_>>();
    output.sort_by(|(_, (_, a)), (_, (_, b))| b.cmp(a));
    let max = **output.iter().map(|(_, (_, a))| a).max().unwrap() as f32;
    for (i, (strat, &sum)) in output {
        let (r, g, b) = hsv_to_rgb((i as f64 * 360.0) / (pool.len() as f64), 1.0, 0.5);
        println!(
            "{}: {}%",
            strat.name().style(Style::new().truecolor(r, g, b)),
            ((sum as f32 / max) * 100.0)
        );
    }

    // Create a chart using the lineplot crate
    let mut chart = Chart::new(280, 120, 0.0, (ROUNDS - 1) as f32);
    let mut chart = chart.y_tick_display(textplots::TickDisplay::Dense);
    let mut shapes = Vec::<Shape>::default();
    let cpy = &delta_sums;
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
                
                cur * mix + last * (1.0 - mix)
            }
        })));
    }

    // Add the line plots from the temp shapes
    for i in 0..pool.len() {
        let (r, g, b) = hsv_to_rgb((i as f64 * 360.0) / (pool.len() as f64), 1.0, 1.0);
        chart = chart.linecolorplot(&shapes[i], rgb::RGB::new(r, g, b));
    }

    chart.display();
}
