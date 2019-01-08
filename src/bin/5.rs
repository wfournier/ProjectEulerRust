fn main() {
    println!("{}", smallest_multiple(1, 20));
}

fn smallest_multiple(min: u64, max: u64) -> u64 {
    let mut n: u64 = 20;
    loop {
        let mut valid: bool = true;
        for i in min..max + 1 {
            if n % i != 0 {
                valid = false;
                break;
            }
        }
        if valid {
            return n;
        } else {
            n += 2;
        }
    }
}
