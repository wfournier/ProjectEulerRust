fn main() {
    let (a, b, c, prod) = solve(1_000);
    println!("{} X {} X {} = {}", a, b, c, prod);
}

fn solve(n: u64) -> (u64, u64, u64, u64) {
    for a in 1..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == n {
                    return (a, b, c, a * b * c);
                }
            }
        }
    }

    (0, 0, 0, 0)
}
