fn fib(n: i32) -> i32 {
    if n > 1 {
        return fib(n - 1) + fib(n - 2);
    }

    return n;
}

fn main() {
    println!("Fibonacci {}", fib(5));
}
