use std::io::{self, Write};
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut rng = thread_rng();

    let length = loop {
        print!("Enter password length (10-128): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(n) = input.trim().parse() {
                if (10..=128).contains(&n) { break n; }
            }
        }
        println!("Invalid length. Please try again.");
    };

    const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const DIGITS: &[u8] = b"0123456789";
    const SPECIAL: &[u8] = b"!@#$%^&*()_+-=";

    let mut password = vec![
        *UPPER.choose(&mut rng).unwrap(),
        *LOWER.choose(&mut rng).unwrap(),
        *DIGITS.choose(&mut rng).unwrap(),
        *SPECIAL.choose(&mut rng).unwrap(),
    ];

    let pool = [UPPER, LOWER, DIGITS, SPECIAL].concat();
    password.extend((0..length - 4).map(|_| *pool.choose(&mut rng).unwrap()));
    password.shuffle(&mut rng);

    println!("\n> Generated Password: {}", String::from_utf8(password).unwrap());
}
