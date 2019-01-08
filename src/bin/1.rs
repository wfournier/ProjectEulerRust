fn main() {
    let mut sum: i32 = 0;
    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum = sum + x;
        }
    }
    println!("{}", sum);
}
