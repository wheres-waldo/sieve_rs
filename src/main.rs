use std::env;

fn eratosthenes(sieve: &[i32]) -> Vec<i32> {
    let mut answer = sieve.to_vec();
    let first = sieve[0];
    answer = cross_out(answer, first);
    answer.insert(0, 2);
    answer
}

fn cross_out(sieve: Vec<i32>, current: i32) -> Vec<i32> {
    let end = {
        let x = *sieve.last().unwrap() as f32;
        x.sqrt().abs() as i32
    };

    if current >= end {
        return sieve;
    }

    let next = *sieve.get(1).unwrap();
    let (mut l, r): (Vec<_>, Vec<_>) = sieve.iter().copied().partition(|&x| x < current.pow(2));
    let shifted: Vec<_> = r.iter().copied().filter(|&x| x % current != 0).collect();

    l.extend_from_slice(&shifted);

    cross_out(l, next)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let sieve: Option<Vec<i32>>;

    sieve = if let Some(i) = args.get(1) {
        if let Ok(i) = i.parse::<i32>() {
            // Skip even numbers since 2 is the only even prime
            Some((3..=i).step_by(2).collect())
        } else {
            None
        }
    } else {
        None
    };

    let answer = match sieve {
        Some(sieve) => eratosthenes(&sieve),
        None => {
            let s: Vec<i32> = (3..=15_485_863).step_by(2).collect();
            eratosthenes(&s)
        }
    };

    println!("{:?}", answer);
    println!("{}", answer.len());
}
