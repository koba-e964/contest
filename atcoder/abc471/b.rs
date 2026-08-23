fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut hm = std::collections::HashMap::new();
    for _ in 0..n {
        let s = getline().trim().to_ascii_lowercase().to_string();
        *hm.entry(s).or_insert(0) += 1;
    }
    println!("{}", hm.values().max().unwrap());
}
