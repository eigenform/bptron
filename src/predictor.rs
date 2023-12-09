
use crate::perceptron::*;
use crate::branch::*;

pub struct Predictor<const L: usize> { 
    pub p: Perceptron<L>,
    pub h: HistoryRegister<L>,
}
impl <const L: usize> Predictor<L> { 
    pub fn new() -> Self { 
        Self { 
            p: Perceptron::new(),
            h: HistoryRegister::new(),
        }
    }

    pub fn reset(&mut self) {
        self.p.reset();
        self.h.reset();
    }

    pub fn predict(&self) -> Outcome { 
        let (_, prediction) = self.p.output(&self.h.data_as::<i8>());
        prediction
    }

    pub fn update(&mut self, outcome: Outcome) {
        self.p.train(&self.h.data_as::<i8>(), outcome);
        self.h.update(outcome);
    }

}


