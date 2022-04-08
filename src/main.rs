use rand::distributions::Distribution;
use rand::prelude::*;
use rand_distr::{Normal, Standard};
// use std::iter::zip;

pub struct Student {
    iq: f64, // N(0, 1)
}
impl Student {
    pub fn new(iq: f64) -> Self {
        Self { iq: iq } // multiplier on passing probability
    }
    pub fn take_test(&self, test: Test) -> f64 {
        // zip(test.difficulties.iter(), test.weights.iter());
        let mut rng: StdRng = StdRng::from_rng(thread_rng()).unwrap();
        test.difficulties
            .iter()
            .zip(test.weights.iter())
            .map(|(&x, &y)| -> f64 {
                (rng.sample::<f64, Standard>(Standard) < (x / self.iq)) as i32 as f64 * y
            })
            .sum::<f64>()
            / test.total_weight
    }
    // pub fn get_answers(&self, test: Test) -> (f64, Vec<bool>) {}
}

pub struct Test {
    difficulties: Vec<f64>, // Bernoulli probabilities of passing the question
    weights: Vec<f64>, // point weights on the test, by default all questions are uniformly weighted
    total_weight: f64,
}
impl Test {
    pub fn new(difficulties: Vec<f64>, weights: Vec<f64>) -> Self {
        assert_eq!(difficulties.len(), weights.len());
        assert!(weights.iter().all(|&x| x >= 0.0));

        Self {
            difficulties: difficulties,
            total_weight: weights.iter().copied().sum(),
            weights: weights,
        }
    }
}

pub fn make_class(size: usize, iq_distribution: impl Distribution<f64>) -> Vec<Student> {
    assert!(size > 0);
    iq_distribution
        .sample_iter(thread_rng())
        .take(size)
        .map(|x: f64| Student::new(x))
        .collect::<Vec<Student>>()
}

fn main() {
    // let class: Vec<_> = make_class(5, StandardNormal);
    let class: Vec<_> = make_class(5, Normal::new(1.0, 0.5).unwrap());
    for student in class {
        println!("{}", student.iq);
    }
}
