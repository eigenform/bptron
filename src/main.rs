///
/// Simple example of branch prediction with a perceptron. 
///
/// Reproduced [albeit haphazardly; I have no idea what I'm doing] from:
///
///     "Neural Methods for Dynamic Branch Prediction" (Jiménez and Lin, 2002)
///     "Fast Path-Based Neural Branch Prediction" (Jiménez, 2003)
///
/// In this context, we're only interested in examining the behavior of a 
/// single perceptron used to learn some pattern of outcomes for a single 
/// branch. Input to the perceptron is the *local history* of outcomes for
/// one particular branch.
///


#[allow(unused_parens)]

mod branch;
mod perceptron;
mod predictor;

use branch::*;
use predictor::*;
use itertools::*;
use std::collections::*;
use rand::prelude::*;

fn main() {

    let mut predictor: Predictor<16> = Predictor::new();
    let mut uncaptured = Vec::new();
    let mut worst = 0;

    // We're going to iterate over the set of all 16-bit branch outcomes
    let mut vals: Vec<u16> = (0x0000..=0xffffu16).collect();

    for val in vals {
        let mut pred_results = Vec::new();

        // Turn some value into a pattern of outcomes
        let pattern = (0..16).map(|idx| Outcome::from_bit(val, idx));

        // Reset the predictor state before testing each pattern. 
        predictor.reset();

        // Repeat the pattern some number of times
        for _ in 0..32 { 
            for outcome in pattern.to_owned() { 
                let prediction = predictor.predict();
                pred_results.push((outcome != prediction) as u8);
                predictor.update(outcome);
            }
        }

        // Get the number of mispredictions
        let misses = pred_results.iter().filter(|x| **x == 1).count();

        // If the last N are correctly predicted, this is probably a good 
        // indication that we've successfully learned the pattern?
        let runs = pred_results.split(|x| *x == 1).filter(|x| x.len() != 0);
        let last = runs.last().unwrap();
        let captured = last.len() >= 128;
        if !captured {
            uncaptured.push(val);
        }

        // If this is the worst-case pattern, print some info
        if misses >= worst {
            println!("{:04x} misses={:4} rate={:.4} captured?={} {:?}", 
                val, misses, ((1024.0 - misses as f64)/1024.0), 
                captured as u8,
                predictor.p.weights(),
            );
            println!("  Mispredictions:");
            for line in pred_results.chunks(32) {
                println!("  {:?}", line);
            }
            worst = misses;
        }

    }

    println!("Uncaptured patterns: {}", uncaptured.len());
    for p in uncaptured.iter().sorted() {
        println!("    {:04x} {:016b}", p, p, );
    }

}
