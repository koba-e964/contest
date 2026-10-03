fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn yn(a: bool) -> &'static str {
    if a {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [a, b] = ints[..] else { panic!() };
    println!("{}", yn(3 * a > 2 * b));
}
