use std::io;

fn main() {
    println!("Welcome to the number theory.");
    println!("1 - Check Prime.");
    println!("2 - GCD/LCM.");
    println!("3 - Collatz step count.");
    println!("4 - Fibonacci.");
    println!("5 - Number conversion.");
    println!("You can type -1 to quit.");

    loop {
        println!("Please enter an operation number : ");
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read the line.");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number {
            0 => break,
            1 => check_prime(),
            _ => println!("Enter a valid operation number"),
        }
    }
}

fn check_prime() {
    println!("Welcome to the prime program :)");
    println!("Enter a number : ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line");

    let n: u32 = n.trim().parse().expect("Please enter a number!");

    if n == 2 {
        println!("{n} is a prime number.");
        return;
    }

    if n < 2 {
        println!("{n} is not a prime number.");
        return;
    }

    for c in 2..=n / 2 {
        if n % c == 0 {
            println!("{n} is not a prime number.");
            return;
        }
    }
    println!("{n} is a prime number.");
}
