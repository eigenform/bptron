
use crate::branch::Outcome;

pub struct Perceptron<const L: usize>  {
    pub weights: [i8; L],
    pub bias: i8,
}
impl <const L: usize> Perceptron<L> {

    // Training threshold. 
    // Papers suggest this constant (based on the history size). 
    const THETA: i8 = ((1.93f32 * (L as f32)) + 14.0f32) as i8;

    pub fn new() -> Self { 
        println!("{}", Self::THETA);
        Self { weights: [0; L], bias: 0, }
    }

    pub fn reset(&mut self) { 
        self.bias = 0;
        self.weights = [0; L];
    }

    pub fn weights(&self) -> &[i8] { &self.weights }

    /// Compute the dot product of the input/weight vectors
    fn dot_product(&self, input: &[i8]) -> i8 {
        assert!(input.len() == L);
        input.iter().zip(self.weights.iter())
            .map(|(i, w)| i.saturating_mul(*w))
            .fold(0, |mut sum, val| { sum = sum.saturating_add(val); sum })
    }

    /// Given some input vector, compute the output value. 
    /// The predicted outcome is determined by the sign of the output.
    pub fn output(&self, input: &[i8]) -> (i8, Outcome) {
        let res = self.dot_product(input).saturating_add(self.bias);
        let out = if res >= 0 { Outcome::T } else { Outcome::N };
        (res, out)
    }

    /// Given some outcome, adjust the weights. 
    pub fn train(&mut self, input: &[i8], outcome: Outcome) {
        let (output, prediction) = self.output(&input);
        let outcome_val: i8 = outcome.into();

        // Training occurs after a misprediction, or when the output value is 
        // below some threshold [Perceptron::THETA]. 
        let miss = (prediction != outcome);
        let output_magnitude = {
            if output > i8::MIN { output.abs() } else { (output + 1).abs() }
        };
        let below_threshold  = (output_magnitude <= Self::THETA);

        // When a bit in the history matches the outcome, increment the 
        // corresponding weight. Otherwise, decrement the corresponding weight.
        if miss || below_threshold {
            self.bias = self.bias.saturating_add(outcome_val);
            for idx in 0..L {
                let adj = if input[idx] == outcome_val { 1 } else { -1 };
                self.weights[idx] = self.weights[idx].saturating_add(adj);
            }
        }
    }
}

