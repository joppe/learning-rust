fn main() {
    let x = 5;

    // shadowed x
    let x = x + 1;

    // shadowed x
    let x = x * 2;

    println!("The value of x is: {}", x);
}
