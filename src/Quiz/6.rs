// Čo vypíše tento program?

fn main() {
    let mut x = Vec::new();
    x.push("James");
    func6(&mut x);
    println!("{}", x[1]);
}

fn func6(v: &mut Vec<&str>) { // mutable reference
    v.push("Bond James");
}
