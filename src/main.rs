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

use crate::{decision::Decision, factors::{ENTITIES_PER_POOL, FULLY_STOLEN_POINTS, HALF_STOLEN_POINTS, NOISE, SHARED_POINTS, STOLEN_PENALTY}, pool::score_pool};

fn main() {
    // Create a pool of multiple boxed strategies so we can duplicate them into their own entity pools
    let mut pool = Vec::<Box<dyn Strategy>>::default();
    pool.push(Box::<Random>::default());
    pool.push(Box::new(Nice));
    pool.push(Box::new(NotNice));
    pool.push(Box::<TitForTat>::default());
    pool.push(Box::<EachNthStealer>::default());
    pool.push(Box::<TwiceGrudge>::default());
    pool.push(Box::<Grudge>::default());
    pool.push(Box::<Prober>::default());

    // Dedupper so we won't run pairs twice
    let mut dedupper = Vec::<(usize, usize)>::new();

    // Check user input to see what strategies are allowed
    println!("Please select which strategies should be used (leave empty if you want all of em)");
    for (i, strat) in pool.iter().enumerate()  {
        println!("{}: {}", char::from_u32(i as u32 + 65).unwrap(), strat.name());    
    }
    let default = "ABCDEFGHIJK".to_string();
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

    println!("{}", "Using the following strategies:".underline().italic());
    let all = pool.iter().map(|x| x.name()).collect::<Vec<&str>>().join(", ");
    println!("{all}");
    println!("{}", "With the following parameters:".underline().italic());
    println!("Shared Points: {}", SHARED_POINTS);
    println!("Half Stolen Points: {}", HALF_STOLEN_POINTS);
    println!("Stolen Penalty: {}", STOLEN_PENALTY);
    println!("Fully Stolen Points: {}", FULLY_STOLEN_POINTS);
    println!("Entities Per Pool: {}", ENTITIES_PER_POOL);
    println!("Noise: {}", NOISE);
    println!("Rounds: {}", ROUNDS); 

    println!("{}", "Theoretical Best/Worst case scenario values:".underline().italic());
    println!("Fully taken advantage of: {}", (ENTITIES_PER_POOL as i64) * STOLEN_PENALTY * (ROUNDS as i64));
    println!("Fully took advantage of: {}", (ENTITIES_PER_POOL as i64) * FULLY_STOLEN_POINTS * (ROUNDS as i64));
    println!("Half/half stealing: {}", (ENTITIES_PER_POOL as i64) * HALF_STOLEN_POINTS * (ROUNDS as i64));
    println!("Nice-maxxing: {}", (ENTITIES_PER_POOL as i64) * SHARED_POINTS * (ROUNDS as i64));

    // Total strategy point sum and point sums gained each round 
    let mut total_sums = vec![0i64; pool.len()];
    let mut delta_sums = vec![[0i64; ROUNDS]; pool.len()];

    let mut rng = StdRand::seed(ClockSeed.next_u64());
    for (i, s1) in pool.iter().enumerate() {
        for (j, s2) in pool.iter().enumerate() {
            if dedupper.contains(&(i,j)) || dedupper.contains(&(j,i)) {
                continue;
            }

            let mut p1 = s1.poolify();
            let mut p2 = s2.poolify();
            dedupper.push((i, j));

            // Make the 2 pools "fight" each other for n number of rounds
            let mut temp: [i64; 2] = [0, 0];
            let mut block_line1 = String::new();
            let mut block_line2 = String::new();
            for r in 0..ROUNDS {
                let mut round_temp: [i64; 2] = [0, 0];
                
                let mut first_entity_decisions = [Decision::Share; 2];
                score_pool(&mut p1, &mut p2, &mut round_temp, &mut first_entity_decisions, &mut rng, r);
                
                total_sums[i] += round_temp[0];
                total_sums[j] += round_temp[1];
                
                temp[0] += round_temp[0];
                temp[1] += round_temp[1];

                delta_sums[i][r] += round_temp[0];
                delta_sums[j][r] += round_temp[1];
                
                block_line1.push_str(&first_entity_decisions[0].color_char('█'));
                block_line2.push_str(&first_entity_decisions[1].color_char('█'));
            }

            // Some cool debugging to see which strategy worked best in this special case
            let name1 = s1.name();
            let name2 = s2.name();
            
            // Calculate percent difference first
            let avg = (temp[0] as f32 + temp[1] as f32) / 2.0;
            let d = temp[0] as f32 - temp[1] as f32;
            let diff = d / avg;

            let max_diff = 0.15f32;
            let percent = (diff / max_diff).clamp(-1.0f32, 1.0f32) * 0.5f32 + 0.5;
            let c1 = rgb::RGB::new(1.0-percent, percent, 0.0).map(|x| (x * 255.0f32) as u8);
            let c2 = rgb::RGB::new(percent, 1.0-percent, 0.0).map(|x| (x * 255.0f32) as u8);

            let name1 = name1.truecolor(c1.r, c1.g, c1.b);
            let name2 = name2.truecolor(c2.r, c2.g, c2.b);
            let line = format!("{} VS {}", name1, name2);
            println!("{line} => ({}, {})", temp[0], temp[1]);
            println!("{0: <30} | {1: <10} {2}", name1, "", block_line1);
            println!("{0: <30} | {1: <10} {2}", name2, "", block_line2);
            println!("");
            
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
