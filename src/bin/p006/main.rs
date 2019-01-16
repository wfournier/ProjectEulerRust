fn main() {
    let min: u64 = 1;
    let max: u64 = 100;
    let (sum_of_square, square_of_sum, solve) = solve(min, max);
    println!("{} - {} = {}", sum_of_square, square_of_sum, solve);
}

fn solve(min: u64, max: u64) -> (u64, u64, u64) {
    let sum_of_square: u64 = (min..max + 1).fold(0, |sum, x| sum + x.pow(2));
    let square_of_sum: u64 = (min..max + 1).fold(0, |sum, x| sum + x).pow(2);

    (sum_of_square, square_of_sum, square_of_sum - sum_of_square)
}
