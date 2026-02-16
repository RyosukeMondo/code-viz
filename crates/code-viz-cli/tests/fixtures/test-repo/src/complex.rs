pub fn complex_function(x: i32, y: i32) -> i32 {
    if x > 0 {
        if y > 0 {
            for i in 0..x {
                if i % 2 == 0 {
                    println!("{}", i);
                }
            }
        } else {
            while y < 0 {
                y += 1;
            }
        }
    }
    x + y
}
