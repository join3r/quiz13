// Čo vypíše tento program?

struct Meno {
    prve_meno: String,
    priezvisko: String,
}

fn main() {
    let join3r = Meno {
        prve_meno: "join3r".into(),
        priezvisko: "bob".into(),
    };
    println!("Volám sa {}, {} {}.", Meno.priezvisko, Meno.prve_meno, Meno.priezvisko);
}