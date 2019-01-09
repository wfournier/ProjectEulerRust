fn main() {
    let (num1, num2, highest) = solve();
    println!("{} x {} = {}", num1, num2, highest);
}

fn solve() -> (u64, u64, u64) {
    let mut num1: u64 = 0;
    let mut num2: u64 = 0;
    let mut highest: u64 = 0;

    for i in 100..1000 {
        for j in 100..1000 {
            let prod: u64 = i * j;
            let s = prod.to_string();
            let rev = s.chars().rev().collect::<String>();

            if s == rev && prod > highest {
                num1 = i;
                num2 = j;
                highest = prod;
            }
        }
    }

    (num1, num2, highest)
}
