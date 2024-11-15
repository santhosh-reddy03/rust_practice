fn main() {
    fibonacci(10);
    println!("");
    println!("Hello, world!");
}

fn fibonacci_loop(n: i32){
    loop{

    }
}
fn fibonacci(i: u32) {
    let mut a = 0;
    let mut b = 1;
    for n in 0..i {
        if n == 0 {
            print!("{} ,", &a);
        } else if  n == 1 {
            print!("{} ,", &b);
        } else {
            let c = &a + &b;
            print!("{} ,", &c);
            a = b;
            b = c;
        }
    }
}