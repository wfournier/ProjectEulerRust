fn main() {
    println!("{}", solve(1_000));
}

fn solve(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for x in 0..n {
        if x % 3 == 0 || x % 5 == 0 {
            sum = sum + x;
        }
    }

    sum
}
