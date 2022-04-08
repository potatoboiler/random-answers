use num::{cast::AsPrimitive, Float, Num};
use std::iter::Sum;

struct Student {
    iq: f64,
}

trait WeightType: Float + Sum + Copy + Ord {}

impl Student {
    pub fn new(iq: f64) -> Self {
        Self { iq: iq }
    }
    pub fn take_test(test: Test<impl Float, impl WeightType>) -> f64 {
        1.0
    }
}

struct Test<D, W>
where
    D: Float,
    W: WeightType,
{
    difficulties: Vec<D>, // Bernoulli probabilities of passing the question
    weights: Vec<W>, // point weights on the test, by default all questions are uniformly weighted
    total_weight: W,
}
impl<D, W> Test<D, W>
where
    D: Float,
    W: WeightType,
{
    pub fn new(difficulties: Vec<D>, weights: Vec<W>) -> Self {
        assert_eq!(difficulties.len(), weights.len());
        // assert!(weights.iter().all(|x| *x > 0));

        let total = weights.iter().copied().sum();

        Self {
            difficulties: difficulties,
            weights: weights,
            total_weight: total,
        }
    }
}
fn main() {
    println!("Hello, world!");
}
