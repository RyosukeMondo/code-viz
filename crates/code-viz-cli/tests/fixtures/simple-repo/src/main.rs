mod utils;

fn main() {
    let result = simple_repo::add(1, 2);
    println!("Result: {}", result);
    utils::print_hello();
}
