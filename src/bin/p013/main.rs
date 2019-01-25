use std::path::Path;

fn main() {
    let list = get_list();
    println!("{:?}", list);
    // println!("{}", solve(list));
}

fn solve(input: Vec<u128>) -> u128 {
    // TODO
    0
}

fn get_list() -> Vec<u128> {
    let data = std::fs::read_to_string("src\\bin\\p013\\numbers.txt").expect("Unable to read file");
    data.split("\n").map(|n| n.parse::<u128>().unwrap()).collect() // FIX: Overflow error
}
