fn main() {
    println!("{}", solve(600_851_475_143));
}

fn solve(num: u64) -> u64 {
    prime_factors(num)
        .iter()
        .fold(0 as u64, |max, x| if x > &max { *x } else { max })
}

fn prime_factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut max: u64 = f64::sqrt(n as f64).trunc() as u64;
    max += 1;

    for i in 3..max {
        if n % i == 0 && is_prime(i) {
            factors.push(i);
        }
    }

    factors
}

fn is_prime(n: u64) -> bool {
    if n == 1 || n % 2 == 0 {
        return false;
    } else if n == 2 {
        return true;
    }

    let max: u64 = f64::sqrt(n as f64).trunc() as u64;
    for i in (3..max + 1).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}
