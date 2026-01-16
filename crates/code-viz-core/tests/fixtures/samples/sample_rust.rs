// Sample Rust file for testing
use std::collections::HashMap;

/// A simple calculator struct
pub struct Calculator {
    memory: HashMap<String, i32>,
}

impl Calculator {
    /// Create a new calculator
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    /// Add two numbers
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    /// Subtract two numbers
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    /// Store a value in memory
    pub fn store(&mut self, key: String, value: i32) {
        self.memory.insert(key, value);
    }

    /// Retrieve a value from memory
    pub fn recall(&self, key: &str) -> Option<i32> {
        self.memory.get(key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        let calc = Calculator::new();
        assert_eq!(calc.subtract(5, 3), 2);
    }
}
