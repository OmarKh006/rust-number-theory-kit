use std::cmp;
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to the number theory.");

    let mut hex_characters = HashMap::new();

    hex_characters.insert("0", '0');
    hex_characters.insert("1", '1');
    hex_characters.insert("2", '2');
    hex_characters.insert("3", '3');
    hex_characters.insert("4", '4');
    hex_characters.insert("5", '5');
    hex_characters.insert("6", '6');
    hex_characters.insert("7", '7');
    hex_characters.insert("8", '8');
    hex_characters.insert("9", '9');
    hex_characters.insert("10", 'A');
    hex_characters.insert("11", 'B');
    hex_characters.insert("12", 'C');
    hex_characters.insert("13", 'D');
    hex_characters.insert("14", 'E');
    hex_characters.insert("15", 'F');

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
            3 => {
                println!("Enter a number : ");

                let mut n = String::new();

                io::stdin()
                    .read_line(&mut n)
                    .expect("Failed to read the line");

                let n: u32 = n.trim().parse().expect("Please enter a number!");
                println!("The step count for {n} is {}", collatz_step_count(n));
            }
            4 => {
                println!("Enter a number : ");

                let mut n = String::new();

                io::stdin()
                    .read_line(&mut n)
                    .expect("Failed to read the line");

                let n: u64 = n.trim().parse().expect("Please enter a number!");
                println!("F({}) is {}", n, fibonacci(n));
            }
            5 => number_conversion(hex_characters.clone()),
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

                let n1: i32 = n1.trim().parse().expect("Please enter a number!");

                println!("Enter second number : ");

                let mut n2 = String::new();

                io::stdin()
                    .read_line(&mut n2)
                    .expect("Failed to read the line");

                let n2: i32 = n2.trim().parse().expect("Please enter a number!");

                println!("the gcd({}, {}) is {}", n1, n2, gcd(n1, n2));
            }
            2 => {
                println!("Enter first number : ");

                let mut n1 = String::new();

                io::stdin()
                    .read_line(&mut n1)
                    .expect("Failed to read the line");

                let n1: i32 = n1.trim().parse().expect("Please enter a number!");

                println!("Enter second number : ");

                let mut n2 = String::new();

                io::stdin()
                    .read_line(&mut n2)
                    .expect("Failed to read the line");

                let n2: i32 = n2.trim().parse().expect("Please enter a number!");

                println!("the lcm({}, {}) is {}", n1, n2, lcm(n1, n2));
            }
            _ => println!("Please Enter a valid operation number"),
        }
    }
}

fn gcd(n1: i32, n2: i32) -> i32 {
    if n1 == 0 && n2 == 0 {
        return 0;
    }
    let greatest = cmp::max(n1, n2);
    let mut gcd = 0;

    for n in 1..=greatest {
        if n1 % n == 0 && n2 % n == 0 {
            gcd = n;
        }
    }

    return gcd;
}

fn lcm(n1: i32, n2: i32) -> i32 {
    if n1 == 0 || n2 == 0 {
        return 0;
    }
    let lcm: i32 = ((n1 * n2).abs()) / gcd(n1, n2);
    return lcm;
}

fn collatz_step_count(mut number: u32) -> u32 {
    if number == 0 {
        return 0;
    }
    let mut step: u32 = 0;
    loop {
        if number == 1 {
            break;
        }
        if number % 2 == 0 {
            number /= 2;
            step = step + 1;
        } else {
            number = (number * 3) + 1;
            step = step + 1;
        }
    }
    return step;
}

fn fibonacci(n: u64) -> u64 {
    let mut last: u64 = 0;
    let mut current: u64 = 1;
    let mut temp: u64;
    if n == 0 {
        return 0;
    }
    for _x in 2..=n {
        temp = current;
        current = current + last;
        last = temp;
    }
    return current;
}

fn number_conversion(hex_characters: HashMap<&str, char>) {
    println!("Welcome the number conversion program :)");

    loop {
        println!("0 - Go Back");
        println!("1 - int to bin");
        println!("2 - int to hex");
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
                println!("Enter a number : ");

                let mut n = String::new();

                io::stdin()
                    .read_line(&mut n)
                    .expect("Failed to read the line");

                let n: i32 = n.trim().parse().expect("Please enter a number!");

                int_to_bin(n);
            }
            2 => {
                println!("Enter a number : ");

                let mut n = String::new();

                io::stdin()
                    .read_line(&mut n)
                    .expect("Failed to read the line");

                let n: i32 = n.trim().parse().expect("Please enter a number!");

                int_to_hex(n, hex_characters.clone());
            }
            _ => println!("Please Enter a valid operation."),
        }
    }
}

fn int_to_bin(mut n: i32) {
    if n == 0 {
        println!("the number {} in binary is 0", n);
        return;
    }
    let mut bin: String = String::new();
    while n > 0 {
        if n % 2 == 0 {
            bin.push('0');
        } else {
            bin.push('1');
        }
        n = n / 2;
    }
    println!(
        "the number {} in binary is {}",
        n,
        bin.chars().rev().collect::<String>()
    );
}

fn int_to_hex(n: i32, hex_characters: HashMap<&str, char>) {
    let base = 16;
    let mut hex_string: String = String::new();

    let float_result = n as f32 / base as f32;
    let mut decimal_part = float_result.fract();
    let mut int_part = float_result.floor();

    let mut remainder = base as f32 * decimal_part;
    let mut s = remainder.to_string();
    let mut slice: &str = &s;

    hex_string.push(hex_characters.get(slice).copied().unwrap_or(' '));

    loop {
        int_part /= base as f32;
        decimal_part = int_part.fract();
        remainder = base as f32 * decimal_part;

        int_part = int_part.floor();

        s = remainder.to_string();
        slice = &s;
        hex_string.push(hex_characters.get(slice).copied().unwrap_or(' '));

        if int_part == 0.00 {
            println!("this number {} in hex is {}", n, hex_string.chars().rev().collect::<String>());
            break;
        }
    }
}
