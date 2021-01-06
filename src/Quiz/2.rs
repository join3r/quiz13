// Čo vypíše tento program?

fn main() {
    let mut a = 10;
    a = func2();
    println!("{}", a);
}

fn func2() -> i32 {
    a + 1
}