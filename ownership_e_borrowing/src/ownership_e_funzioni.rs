/* I meccanismi che regolano il passaggio di un valore a una funzione sono simili a quelli
dell’assegnazione di un valore a una variabile. Passando una variabile a una funzione,
questa viene spostata o copiata, proprio come fa l’assegnazione.*/

pub fn main() {
    let s = String::from("hello"); // 's' entra nello scope
    
    prende_ownership(s);                     // il valode di 's' viene spostato nella funzione
                                             // qui smette di essere valido
    
    let x = 5;                          // x entra nello scope

    duplica(x);                              // siccome i32 implementa il tratto Copy,
                                             // s NON viene spostato nella funzione,
                                             // quindi dopo può ancora essere usata.
}   // Qui, x esce dallo scope

fn prende_ownership(una_stringa: String) { // una_stringa entra nello scope
    println!("{una_stringa}");
} // qui esce dallo scope e "Drop" viene chiamato. Memoria rilasciata.

fn duplica(un_integer: i32) {
    println!("{un_integer}");
} // Qui un_integer (copia del integer passato) esce dallo scope e viene quindi distrutto.