fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<i64>().unwrap();
    println!("{}", (1 << n) - 2 * n);
}
