fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut vals: Vec<i32> = getline().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    vals.sort();
    println!("{}", if vals[0] == vals[2] || vals[0] + vals[1] == vals[2] {
        "Yes"
    } else {
        "No"
    });
}
