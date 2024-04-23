mod decision;
mod factors;
mod pool;
mod strats;
use factors::ROUNDS;
use rgb::ComponentMap;
use strats::*;

use hsv::hsv_to_rgb;
use owo_colors::{OwoColorize, Style};
use std::cmp::Ordering;
use strats::Strategy;
use textplots::{Chart, ColorPlot, Shape, TickDisplayBuilder};
use tinyrand::{Rand, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use crate::{decision::{color_f32_char, Decision}, factors::{print_params, ENTITIES_PER_POOL, FULLY_STOLEN_POINTS, HALF_STOLEN_POINTS, HISTOGRAM_ENTITY_COUNT, NOISE, SHARED_POINTS, STOLEN_PENALTY}, pool::score_pool};

fn main() {
    // Create a pool of multiple boxed strategies so we can duplicate them into their own entity pools
    let mut pool = Vec::<Box<dyn Strategy>>::default();
    pool.push(Box::<Random>::default());
    pool.push(Box::<Nice>::default());
    pool.push(Box::<NotNice>::default());
    pool.push(Box::<TitForTat>::default());
    pool.push(Box::<TitForTwoTat>::default());
    pool.push(Box::<EachNthStealer>::default());
    pool.push(Box::<Grudge>::default());
    pool.push(Box::<Prober>::default());

    // Dedupper so we won't run pairs twice
    let mut dedupper = Vec::<(usize, usize)>::new();

    // Check user input to see what strategies are allowed
    println!("Please select which strategies should be used (leave empty if you want Nice, NotNice, and TitForTwoTat)");
    for (i, strat) in pool.iter().enumerate()  {
        println!("{}: {}", char::from_u32(i as u32 + 65).unwrap(), strat.name());    
    }
    let default = "BCE".to_string();
    let input = std::io::stdin();
    let mut allowed = input.lines().next().unwrap_or(Ok(default.clone())).unwrap_or(default.clone());
    if allowed.is_empty() {
        allowed = default.clone();
    }    

    let len = pool.len();
    for i in (0..len).rev() {
        if !allowed.contains(char::from_u32(i as u32 + 65).unwrap()) {
            pool.remove(i);
        }
    }
    
    print_params(&pool);

    // Total strategy point sum and point sums gained each round 
    let mut total_sums = vec![0i64; pool.len()];
    let mut delta_sums = vec![[0i64; ROUNDS]; pool.len()];

    let mut rng = StdRand::seed(ClockSeed.next_u64());
    for (i, s1) in pool.iter().enumerate() {
        for (j, s2) in pool.iter().enumerate() {
            if dedupper.contains(&(i,j)) || dedupper.contains(&(j,i)) {
                continue;
            }

            dedupper.push((i, j));
            
            // Create the pools for the two strategies
            let mut pool_rng = StdRand::seed(1234);
            let mut p1 = s1.poolify(&mut pool_rng);
            let mut p2 = s2.poolify(&mut pool_rng);

            // Setup temp vars for round play-off
            let mut playoff_points: [f32; 2] = [0.0, 0.0];
            let mut histograms = [String::new(), String::new()];
            let mut decision_sums = [0.0f32; 2];
            let local_to_global_indices = [i, j];
            
            // Make the 2 pools "fight" each other for n number of rounds
            for r in 0..ROUNDS {
                let mut temp_round_points: [i64; 2] = [0, 0];                
                let mut decisions = [0.0f32; 2];

                score_pool(&mut p1, &mut p2, &mut temp_round_points, &mut decisions, HISTOGRAM_ENTITY_COUNT, &mut rng, &mut pool_rng, r);
                
                // So point summing logic here
                for local in 0..2 {
                    let global = local_to_global_indices[local];
                    total_sums[global] += temp_round_points[local];
                    delta_sums[global][r] += temp_round_points[local];
                    playoff_points[local] += temp_round_points[local] as f32;
                    histograms[local].push_str(&color_f32_char(decisions[local] / HISTOGRAM_ENTITY_COUNT as f32, '█'));
                    decision_sums[local] += decisions[local];                    
                }
            }

            // Some cool debugging to see which strategy worked best in this special case
            let names = [s1.name(), s2.name()];
            let avg = playoff_points.iter().sum::<f32>() / 2.0;
            let diff = (playoff_points[1] - playoff_points[0]) / avg;

            let line = format!("{} VS {}", names[0], names[1]);
            println!("{line} => ({}, {})", playoff_points[0], playoff_points[1]);

            for local in 0..2 {
                let max_diff = 0.15f32;
                let mut percent = (diff / max_diff).clamp(-1.0f32, 1.0f32) * 0.5f32 + 0.5;

                // swap percent if we're the second one (loss)
                if local == 1 {
                    percent = 1.0 - percent;
                }

                let red = ((1.0-percent) * 255.0f32) as u8;
                let green = (percent * 255.0f32) as u8;
                let colored = names[local].truecolor(red, green, 0);
                println!("{0: <20} (avg: {1: <6}) | {2: <1} {3}", colored, decision_sums[local], "", histograms[local]);                  
            }            
        }
    }

    // Very ugly code to try to find relative percentage and to sort the strats
    println!("{}", "Final Point Outcome & Graph:".underline().italic());
    let mut output = pool.iter().zip(total_sums.iter()).enumerate().collect::<Vec<_>>();
    output.sort_by(|(_, (_, a)), (_, (_, b))| b.cmp(a));
    let max = **output.iter().map(|(_, (_, a))| a).max().unwrap() as f32;
    for (i, (strat, &sum)) in output {
        let (r, g, b) = hsv_to_rgb((i as f64 * 360.0) / (pool.len() as f64), 1.0, 0.5);
        println!(
            "{}: {}% ({} total)",
            strat.name().style(Style::new().truecolor(r, g, b)),
            ((sum as f32 / max) * 100.0),
            sum
        );
    }

    // Create a chart using the lineplot crate
    let mut chart = Chart::new(280, 120, 3.0, (ROUNDS - 1) as f32);
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
