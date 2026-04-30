use std::io;

fn main() {
    println!("***********************************************************");
    println!("* Farenheit - Celsius converter!                          *");
    println!("***********************************************************");

    let choose: u8 = loop {
        println!("Options:");
        println!("1) Convert Fahrenheit into Celsius;");
        println!("2) Convert Celsius into Farenheit.");

        let mut choose = String::new();

        io::stdin().read_line(&mut choose).expect("Reading error!");

        match choose.trim().parse() {
            Ok(num) => {
                if num != 1 && num != 2 {
                    eprintln!("Insert a valid option!");
                    continue;
                } else {
                    break num;
                }
            }
            Err(_) => {
                eprintln!("Insert a valid option!");
                continue;
            }
        };
    };

    if choose == 1 {
        convert_fahrenheit_into_celsius();
    } else {
        convert_celsius_into_fahrenheit();
    }
}

fn ask_a_valid_float(msg: &str) -> f64 {
    loop {
        println!("{msg}");

        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Reading error.");

        match fahrenheit.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                eprintln!("Insert a valid temperature!");
            }
        }
    }
}

fn convert_fahrenheit_into_celsius() {
    let fahrenheit: f64 = ask_a_valid_float("Insert a temperature (°F): ");
    let celsius = (fahrenheit - 32.0) * (5.0 / 9.0);
    println!("{fahrenheit} °F are {celsius} °C.")
}

fn convert_celsius_into_fahrenheit() {
    let celsius: f64 = ask_a_valid_float("Insert a temperature (°C): ");
    let fahrenheit = (celsius * (9.0 / 5.0)) + 32.0;
    println!("{celsius} °C are {fahrenheit} °F.")
}
