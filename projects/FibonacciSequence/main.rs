fn main() {
    let n = 10; // Generate the first 10 Fibonacci numbers
    for i in 0..n {
        println!("{}", fibonacci(i));
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
