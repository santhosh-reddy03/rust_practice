use std::io;

fn main() {
    println!("fibonacci numbers using for and conditionals\n");
    fibonacci(10);
    println!("\n\nfibonacci using loop");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read count for fibonacci number");
    match input.trim().parse() {
        Ok(num) => fibonacci_loop(num),
        Err(_) => println!("expected integer input!!!"),
    }

    println!("\ntemperature conversion");
    let fahreinheit = 212;
    println!("\ncelcius {}", get_temperature_in_celcius(&fahreinheit));
}

fn fibonacci_loop(n: i32) {
    let mut a = 0;
    let mut b = 1;
    let mut counter = 0;
    let mut c = 0;
    loop {
        if counter == 0 {
            print!("\n{} ,", a);
        } else if counter == 1 {
            print!("{} ,", b);
        } else if counter < n {
            c = &a + &b;
            print!("{} ,", c);
            a = b;
            b = c;
        } else {
            break println!("{}", c);
        }
        counter = counter + 1;
    }
}

fn fibonacci(i: u32) {
    let mut a = 0;
    let mut b = 1;
    for n in 0..i {
        if n == 0 {
            print!("{} ,", &a);
        } else if n == 1 {
            print!("{} ,", &b);
        } else {
            let c = &a + &b;
            print!("{} ,", &c);
            a = b;
            b = c;
        }
    }
}

fn get_temperature_in_celcius(temp_in_fahrenheit: &u32) -> f64 {
    let temp_in_celcius: f64 = ((temp_in_fahrenheit - 32) as f64) * (5.0 / 9.0);
    temp_in_celcius
}
