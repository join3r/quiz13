// Čo vypíše tento program?

enum Rozhodnutie {
    Ano,
    Nie,
}

fn main() {
    let odpoved: Rozhodnutie = Rozhodnutie::Ano;
    let rozhodnutie = Rozhodnutie::Nie;
    if odpoved == Rozhodnutie::Ano {
        println!("Jeho odpoveď bola áno");
    };
    if odpoved == Rozhodnutie::Nie {
        println!("Jeho odpoveď bola nie");
    }
}