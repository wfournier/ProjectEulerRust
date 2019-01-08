fn main() {
    make_fib(4_000_000);
}

fn make_fib(max: i64){
    let mut fib = vec![1,2];
    let mut sum = 0;

    loop {
        let n = fib.len();
        let last = fib[n-1];
        
        if last > max {
            break;
        }
        else if last % 2 == 0 {
            sum += last;
        }

        let prev = fib[n-2];
        fib.push(prev + last);
    }
    println!("{}", sum);
}