use std::io;

fn main() {
    println!("Insert the number of Fibonacci to print: ");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Reading error.");

    let num: u64 = num.trim().parse().expect("Wrong input!\n");

    let mut prev: u64 = 0;
    let mut pre_prev: u64 = 1;
    
    for i in 1..=num {
        let fib = pre_prev + prev;
        println!("{i}) {pre_prev} + {prev} = {fib}");
        pre_prev = prev;
        prev = fib;
    }
}
