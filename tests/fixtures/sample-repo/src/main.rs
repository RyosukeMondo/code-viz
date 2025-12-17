// Sample Rust file for E2E testing
fn main() {
    println!("Hello, world!");
    let result = add(2, 3);
    println!("2 + 3 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
