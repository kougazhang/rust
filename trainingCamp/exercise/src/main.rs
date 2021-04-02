use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    let num = args[1].parse().unwrap();

    if num < 2 {
        println!("{} is not prime number", num);
        return;
    }

    for n in 2..num {
        if num % n == 0 {
            println!("{} is not prime number", num);
            return;
        }
    }

    println!("{} is prime number", num);
}
