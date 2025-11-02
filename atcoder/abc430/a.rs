fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [a, b, c, d] = ints[..] else { panic!() };
    println!("{}", if c >= a && d < b {
        "Yes"
    } else {
        "No"
    });
}
