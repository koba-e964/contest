// Depends on: dfa.rs
include!("dfa.rs");

struct Add;

impl<S> ActionMonoid<S> for Add {
    type T = i64;
    fn add(&self, x: i64, y: i64) -> i64 {
        x + y
    }
    fn act(&self, x: i64, _y: S) -> i64 {
        x
    }
    fn zero(&self) -> i64 { 0 }
    fn one(&self) -> i64 { 1 }
}

struct GtZero;

impl DFA<i32> for GtZero {
    fn size(&self) -> usize {
        2
    }
    fn trans(&self, zero: usize, c: i32) -> Option<usize> {
        if zero == 1 && c < 0 {
            return None;
        }
        Some(zero & if c == 0 { 1 } else { 0 })
    }
    fn init(&self) -> Vec<usize> {
        vec![1]
    }
    fn is_final_state(&self, state: usize) -> bool {
        state == 0
    }
}

struct DeltaZero(usize);

impl DFA<i32> for DeltaZero {
    fn size(&self) -> usize {
        2 * self.0
    }
    fn trans(&self, delta: usize, c: i32) -> Option<usize> {
        let delta = delta as i32;
        if delta + c < 0 || delta + c >= 2 * self.0 as i32 {
            return None;
        }
        Some((delta + c) as usize)
    }
    fn init(&self) -> Vec<usize> {
        vec![self.0]
    }
    fn is_final_state(&self, state: usize) -> bool {
        state == self.0
    }
}
