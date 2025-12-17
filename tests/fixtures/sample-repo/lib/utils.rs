// Utility functions for sample repo
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

pub struct Calculator {
    value: f64,
}

impl Calculator {
    pub fn new(initial: f64) -> Self {
        Self { value: initial }
    }

    pub fn add(&mut self, n: f64) -> &mut Self {
        self.value += n;
        self
    }

    pub fn subtract(&mut self, n: f64) -> &mut Self {
        self.value -= n;
        self
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}
