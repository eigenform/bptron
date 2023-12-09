
pub trait HasBits {
    fn bit(self, x: usize) -> bool;
}
impl HasBits for u8 {
    fn bit(self, x: usize) -> bool { (self & (1 << x)) != 0 }
}
impl HasBits for u16 {
    fn bit(self, x: usize) -> bool { (self & (1 << x)) != 0 }
}
impl HasBits for u32 {
    fn bit(self, x: usize) -> bool { (self & (1 << x)) != 0 }
}
impl HasBits for u64 {
    fn bit(self, x: usize) -> bool { (self & (1 << x)) != 0 }
}


/// A branch outcome. 
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcome { T, N }
impl Outcome {
    pub fn from_bit<T: HasBits>(val: T, x: usize) -> Self { 
        Self::from(val.bit(x))
    }
}
impl From<i8> for Outcome {
    fn from(x: i8) -> Self { 
        match x { 
            1 => Self::T,
            -1 => Self::N,
            _ => unreachable!(),
        }
    }
}
impl From<bool> for Outcome {
    fn from(x: bool) -> Self { 
        match x { true => Self::T, false => Self::N, }
    }
}
impl From<Outcome> for i8 { 
    fn from(x: Outcome) -> Self { 
        match x { Outcome::T => 1, Outcome::N => -1 }
    }
}



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Branch {
    addr: usize,
    tgt: usize,
    outcome: Outcome,
}

/// A history of branch outcomes.
#[derive(Debug)]
pub struct HistoryRegister<const L: usize> {
    data: [Outcome; L],
}
impl <const L: usize> HistoryRegister<L> {
    pub fn new() -> Self {
        Self { data: [Outcome::N; L], }
    }

    pub fn reset(&mut self) {
        self.data = [Outcome::N; L];
    }

    pub fn data(&self) -> &[Outcome] {
        &self.data
    }

    pub fn data_as<T: From<Outcome> + Copy + Default>(&self) -> [T; L] {
        let mut res = [T::default(); L];
        for idx in 0..L {
            res[idx] = self.data[idx].into();
        }
        res
    }

    pub fn update(&mut self, next: Outcome) {
        self.data.rotate_right(1);
        self.data[0] = next;
    }
}


