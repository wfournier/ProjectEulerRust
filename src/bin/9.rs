fn main() {
    println!("{}", solve(1000));
}

fn solve(n: u64) -> u64 {
    for a in 1..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == n {
                    return a * b * c;
                }
            }
        }
    }
    0
}
