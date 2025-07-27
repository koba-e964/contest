fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let ints = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let m = ints[1];
    let a: i32 = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .sum();
    println!("{}", if a <= m { "Yes" } else { "No" });
}
