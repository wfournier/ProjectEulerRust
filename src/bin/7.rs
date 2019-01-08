fn main() {
    let target: u64 = 10_001;
    let mut primes: u64 = 1;

    let mut pos: u64 = 3;
    while primes != target {
        if is_prime(pos) {
            primes += 1;
        }
        if primes != target {
            pos += 2;
        }
    }

    println!("{}: {}", primes, pos);
}

fn is_prime(x: u64) -> bool {
    if x == 1 || x % 2 == 0 {
        return false;
    } else if x == 2 {
        return true;
    }

    let boundary: u64 = (x as f64).sqrt().floor() as u64;
    for i in (3..boundary + 1).step_by(2) {
        if x % i == 0 {
            return false;
        }
    }

    return true;
}
