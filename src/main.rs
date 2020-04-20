use std::env;

fn eratosthenes(sieve: Vec<i32>) -> Vec<i32> {
    let mut answer = cross_out(sieve.to_vec());
    answer.insert(0, 2);
    answer
}

fn cross_out(sieve: Vec<i32>) -> Vec<i32> {
    let end = (*sieve.last().unwrap() as f32).sqrt().abs() as i32;

    let mut answer = sieve.clone();
    let mut current = *sieve.first().unwrap();
    let mut i = 1;

    while current <= end {
        let (mut l, r): (Vec<_>, Vec<_>) =
            answer.iter().copied().partition(|&x| x < current.pow(2));
        let shifted: Vec<_> = r.iter().copied().filter(|&x| x % current != 0).collect();

        l.extend_from_slice(&shifted);
        answer = l;
        current = *answer.get(i).unwrap();
        i += 1;
    }

    answer
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
        Some(sieve) => eratosthenes(sieve),
        None => eratosthenes((3..=15_485_863).step_by(2).collect()),
    };

    println!("{:?}", answer);
    println!("{}", answer.len());
}
