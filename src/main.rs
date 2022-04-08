struct Student {
    iq: f64,
}
impl Student {
    pub fn new(iq: f64) -> Self {
        Self { iq: iq }
    }
    pub fn take_test(test: Test) -> f64 {
        1.0
    }
}

struct Test {
    difficulties: Vec<f64>, // Bernoulli probabilities of passing the question
    weights: Vec<f64>, // point weights on the test, by default all questions are uniformly weighted
    total_weight: f64,
}
impl Test {
    pub fn new(difficulties: Vec<f64>, weights: Vec<f64>) -> Self {
        assert_eq!(difficulties.len(), weights.len());
        assert!(weights.iter().all(|x| *x > 0.0));

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
