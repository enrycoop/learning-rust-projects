/*
   Anche se funziona, prendere e cedere la ownership con ogni funzione è un po' faticoso.
   Cosa succede se vogliamo consentire a una funzione di utilizzare un valore ma non di
   prenderne la ownership?

   Rust ci permette di ritornare più valori utilizzando una tupla.
 */

pub fn main() {

    // prestito di una reference
    let s1 = String::from("Artemis");

    let lung = calcola_lunghezza(&s1);

    println!("La lunghezza di '{s1}' è {lung}.");

    // modifica di una reference presa in prestito

    let mut s = s1.clone();

    cambia(&mut s);

    println!("{s}");


    /* Le reference mutabili hanno una grande restrizione:
    se hai una reference mutabile a un valore, non puoi avere altri reference a quel valore.*/

    let s3 = String::from("Artemis II");

    // è possibile avere più reference immutabili contemporaneamente
    let r1 = &s;
    let r2 = &s;

    println!("{s3} = {r1} {r2}");

    // le varibili r1 e r2 non verranno più usate dopo questo punto

    let r3 = &mut s;
    r3.push_str(", Good Luck!");
    println!("{r3}");



    /* REFERENCE PENDENTI
    nei linguaggi con puntatori, è facile creare erroneamente un puntatore pendente, cioè
    un puntatore che fa riferimento a una posizione in memoria non più valido, perchè quella
    memoria assegnata a quella variabile è stata liberata, ma non si è provveduto a cancellare
    anche il puntatore che per l'appunto rimane pendente puntando a qualcosa che non è più
    disponibile. In Rust al contrario, il compilatore garantisce che i Reference non diverranno
    mai pendenti: se si ha un reference ad alcuni dati, il compilatore ci impedirà di usare
    quel reference dopo che i dati sono usciti dallo scope.
    */

    let reference_a_nulla = non_pendente();
    println!("{reference_a_nulla}");

}

fn calcola_lunghezza(s: &String) -> usize {
    let lunghezza = s.len(); // len() resituisce la lunghezza di una String

    lunghezza
}

fn cambia(una_stringa: &mut String) {
    una_stringa.push_str(", good luck!");
}

/*
fn pendente() -> &String { // pendente ritorna una reference a String
    let s = String::from("Apollo 13"); // s è una string nuova

    &s // ritorniamo una reference alla String s
} // Qui s esce dallo scope e viene cancellata, così come la memoria assegnatale. Pericolo!
*/
fn non_pendente() -> String { // Soluzione è restituire direttamente una String
    let s = String::from("Apollo 13"); // s è una string nuova

    s // ritorniamo una reference alla String s
}