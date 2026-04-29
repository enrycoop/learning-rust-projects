// module cmp, enum Ordering
use std::cmp::Ordering;
// module io
use std::io;

// trait Rng per poter usare tutti quelli che implementano il trait Rng
use rand::Rng;

fn main() {
    println!("Indovina il numero!");

    let numero_segreto = rand::thread_rng().gen_range(1..=100);

    println!("Il numero segreto è: {numero_segreto}");

    loop {
        println!("Inserisci la tua ipotesi.");

        let mut ipotesi = String::new();

        io::stdin()
            .read_line(&mut ipotesi)
            .expect("Errore di lettura");

        // shadowing di ipotesi, cambio del tipo
        let ipotesi: u32 = match ipotesi.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Hai ipotizzato: {ipotesi}");

        match ipotesi.cmp(&numero_segreto) {
            Ordering::Less => println!("Troppo piccolo!"),
            Ordering::Greater => println!("Troppo Grande!"),
            Ordering::Equal => {
                println!("Hai indovinato!");
                break;
            }
        }
    }
}
