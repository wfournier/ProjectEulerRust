fn main() {
    println!("{}", solve(2_000_000));
}

fn solve(n: u64) -> u64 {
    let mut sum: u64 = 2;
    for i in (3..n).step_by(2) {
        if is_prime(i) {
            sum += i;
        }
    }
    sum
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
