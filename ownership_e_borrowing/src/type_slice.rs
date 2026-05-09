/* TYPE SLICE
Le slice (sezioni, fette, porzioni in italiano) ti permettono di fare riferimento (un reference) a
una sequenza contigua di elementi in una collezione. Una slice è una tipologia di reference,
quindi non ha ownership.
*/


/*
 problema di programmazione: scrivi una funzione che prenda una stringa di parole separate da spazi
 e restituisca la prima parola che trova in quella stringa. Se la funzione non trova uno spazio
 nella stringa, l'intera stringa deve essere considerata come una sola parola.
*/
fn prima_parola(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &lettera) in bytes.iter().enumerate() {
        if lettera == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn main() {
    let s = String::from("Space Sweepers");

    let parola = prima_parola(&s[..]); // 'parola' riceverà il valore 5

    //s.clear();
    // ricorda le regole del borrowing: se abbiamo una reference immutabile a qualcosa (es. slice)
    // non possiamo anche prendere una reference mutabile.
    println!("la prima parola è: {parola}");


    // altre slice

    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}