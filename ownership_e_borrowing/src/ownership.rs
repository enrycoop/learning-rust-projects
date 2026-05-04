/* La ownership è un insieme di regole che disciplinano la gestione della memoria 
da parte di un programma Rust.*/

pub fn main() {
    {
        println!("Modifica della String, allocazione nello heap a runtime.");
        let mut s = String::from("ciao");

        s.push_str(", mondo!");

        println!("{s}");
    } // fine dello scope, s viene deallocata chiamando automaticamente drop

    {
        println!("Interazione tra variabili e dati con Move.");
        let s1 = String::from("ciao");
        let _s2 = s1; // moved

        // println!("{s1}, world!"); difatti questa riga non è valida

        // Per scelta progettuale, Rust non creerà mai automaticamente copie
        // "profonde" dei tuoi dati. Pertanto, si può presupporre che qualsiasi
        // copia automatica sia poco dispendiosa in termini prestazionali e di memroia
    }

    {
        println!("Interazione tra Variabili e Dati con Clone.");
        // Se vogliamo fare una copia profonda possiamo usare un metodo comune chiamato clone.
        let s1 = String::from("hello");
        let s2 = s1.clone(); // una chiamata a clone può essere dispendiosa

        println!("s1 = {s1}, s2 = {s2}");
    }

    {
        println!("Duplicare dati sullo stack.");
        let x = 5;
        let _y = x; // qui viene copiato il valore poichè piccolo e fisso
        // Rust ha una annotazione speciale chiata trait "Copy" che possiamo appiccicare sui type
        // memorizzati nello stack. Se un type implementa il trait "Copy", le variabili che
        // lo utilizzano non si spostano, ma vengono semplicemente copiate.

        // ATTENZIONE: Rust non ci permette di annotare un type con Copy se il type, o una qualsiasi
        // delle sue parti ha implementato il trait Drop.
    }

}
