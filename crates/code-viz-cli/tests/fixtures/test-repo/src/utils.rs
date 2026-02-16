use crate::lib::{add, multiply};

pub fn calculate(a: i32, b: i32) -> i32 {
    add(a, b) + multiply(a, b)
}
