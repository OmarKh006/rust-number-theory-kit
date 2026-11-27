use std::cmp;
use std::io;

fn main() {
    println!("Welcome to the number theory.");

    loop {
        println!("1 - Check Prime.");
        println!("2 - GCD/LCM.");
        println!("3 - Collatz step count.");
        println!("4 - Fibonacci.");
        println!("5 - Number conversion.");
        println!("You can type 0 to quit.");
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
            2 => gcd_lcm(),
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

fn gcd_lcm() {
    println!("Welcome to the GCD/LCM program :)");

    loop {
        println!("0 - Go Back");
        println!("1 - GCD");
        println!("2 - LCM");
        println!("Choose : ");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read the line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match n {
            0 => break,
            1 => {
                println!("Enter first number : ");

                let mut n1 = String::new();

                io::stdin()
                    .read_line(&mut n1)
                    .expect("Failed to read the line");

                let n1: u32 = n1.trim().parse().expect("Please enter a number!");

                println!("Enter second number : ");

                let mut n2 = String::new();

                io::stdin()
                    .read_line(&mut n2)
                    .expect("Failed to read the line");

                let n2: u32 = n2.trim().parse().expect("Please enter a number!");

                gcd(n1, n2);
            }
            _ => println!("Please Enter a valid operation number"),
        }
    }
}

fn gcd(n1: u32, n2: u32) {
    if n1 == 0 && n2 == 0 {
        println!("The gcd({}, {}) is 0", n1, n2);
        return;
    }
    let greatest = cmp::max(n1, n2);
    let mut gcd = 0;

    for n in 1..=greatest {
        if n1 % n == 0 && n2 % n == 0 {
            gcd = n;
        }
    }

    println!("The gcd({}, {}) is {gcd}", n1, n2);
}
