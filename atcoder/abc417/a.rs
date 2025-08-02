fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let ints = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = ints[0];
    let a = ints[1];
    let b = ints[2];
    let s = getline().trim().to_string();
    for (i, c) in s.chars().enumerate() {
        if i >= a && i < n - b {
            print!("{c}");
        }
    }
    println!();
}
