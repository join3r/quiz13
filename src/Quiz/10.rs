// Tento program si prejdeme spolu

use std::mem::size_of_val;

// size_of_val() Returns the size of the pointed-to value in bytes.

fn main() {
    println!("bool - {} bytes", size_of_val(&true));
    println!("u8 - {} bytes", size_of_val(&10_u8));
    println!("i8 - {} bytes", size_of_val(&100_u8));
    println!("u16 - {} bytes", size_of_val(&100_u16));

    let empty = Emtpy;
    println!("Empty - {} bytes", size_of_val(&empty));
    let doga = Pes { rokov: 10 };
    println!("Pes - {} bytes", size_of_val(&doga));

    let cisla = Cisla::C1;
    println!("Cisla - {} bytes", size_of_val(&cisla));

    let cisla2 = Cisla2::C1;
    println!("Cisla2 - {} bytes", size_of_val(&cisla2));
}

struct Emtpy;

#[allow(dead_code)]
struct Pes {
    rokov: u16,
}

enum Cisla {
    C1,
}

#[allow(dead_code)]
enum Cisla2 {
    C1,
    C2,
}