// i valori di ritorno possono anch'essi trasferire la ownership.
/* La ownership di una variabile segue ogni volta lo stesso schema: assegnare un valore
 a un'altra variabile la sposta. Quando una variabile che include dati nell'heap esce
 dallo scope, il valore verrà cancellato da drop a meno che la ownership dei dati 
 non sia stata spostata ad un'altra variabile. */

pub fn main() {
    let s1 = cede_ownership(); // il valore di ritorno viene spostato in s1
    println!("{s1}");

    let s2 = String::from("hello");

    let s3 = prende_e_resituisce(s2); // s2 viene spostata (moved)

    println!("{s3}");

} /* s3: drop | s2 era stata spostata quindi nulla | s1: drop*/

fn cede_ownership() -> String {
    // Sposterà il proprio valore di ritorno alla funzione chiamante.
    let una_stringa = String::from("yours");

    una_stringa
}

fn prende_e_resituisce(altra_stringa: String) -> String {
    // altra_stringa entra nello scope
    altra_stringa // viene restituita e spostata (move) alla funzione chiamante.
}