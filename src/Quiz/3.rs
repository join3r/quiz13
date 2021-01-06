// Čo vypíše tento program?

fn main() {
    let b: String = "Hello".into();
    func3(b);
    println!("{}", b);
}

fn func3(a: String) {
    println!("World!");
}