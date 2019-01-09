fn main() {
    println!("{}", solve(4_000_000));
}

fn solve(max: u64) -> u64 {
    let mut fib = vec![1, 2];
    let mut sum: u64 = 0;

    loop {
        let n = fib.len();
        let last = fib[n - 1];

        if last > max {
            break;
        } else if last % 2 == 0 {
            sum += last;
        }

        let prev = fib[n - 2];
        fib.push(prev + last);
    }
    sum
}
