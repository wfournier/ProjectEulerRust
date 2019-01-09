fn main() {
    println!("{}", solve(10_001));
}

fn solve(n: u64) -> u64 {
    let mut pos: u64 = 1;

    let mut curr: u64 = 3;
    while pos != n {
        if is_prime(curr) {
            pos += 1;
        }
        if pos != n {
            curr += 2;
        }
    }

    curr
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
