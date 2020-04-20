use std::env;

fn eratosthenes(sieve: Vec<i32>) -> Vec<i32> {
    sieve
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let sieve: Option<Vec<i32>>;

    sieve = if let Some(i) = args.get(1) {
        if let Ok(i) = i.parse::<i32>() {
            Some((1..=i).collect())
        } else {
            None
        }
    } else {
        None
    };

    let answer = match sieve {
        Some(sieve) => eratosthenes(sieve),
        None => eratosthenes((1..=30).collect()),
    };

    println!("{:?}", answer);
}
