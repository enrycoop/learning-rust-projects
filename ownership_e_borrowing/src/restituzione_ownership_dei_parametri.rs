/*
   Anche se funziona, prendere e cedere la ownership con ogni funzione è un po' faticoso.
   Cosa succede se vogliamo consentire a una funzione di utilizzare un valore ma non di
   prenderne la ownership?

   Rust ci permette di ritornare più valori utilizzando una tupla.
 */

pub fn main() {
    let s1 = String::from("Artemis");

    let lung = calcola_lunghezza(&s1);

    println!("La lunghezza di '{s1}' è {lung}.");
}

fn calcola_lunghezza(s: &String) -> usize {
    let lunghezza = s.len(); // len() resituisce la lunghezza di una String

    lunghezza
}