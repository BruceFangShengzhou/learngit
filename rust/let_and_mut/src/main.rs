fn main() {
    let a = "123";
    let mut b = 4;

    println!("a is {}, b is {}", a, b);

    let a = "456";
    const C: i32 = 55;
    b = 44;
    println!("a is {}, b is {}, c is {}", a, b, C);
}
