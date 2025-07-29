fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let a: Vec<i32> = getline().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    for i in 1..=13 {
        let mut b = a.clone();
        b.push(i);
        b.sort();
        if b[0] == b[2] && b[3] == b[4] && b[2] != b[3] {
            println!("Yes");
            return;
        }
        if b[0] == b[1] && b[2] == b[4] && b[1] != b[2] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
