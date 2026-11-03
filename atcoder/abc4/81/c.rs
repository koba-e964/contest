fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let q = getline().trim().parse::<i32>().unwrap();
    let mut reg = std::collections::HashMap::new();
    for _ in 0..q {
        let s = getline().trim().to_string();
        let idx = if let Some(&idx) = reg.get(&s) {
            idx
        } else {
            let idx = reg.len() + 1;
            reg.insert(s, idx);
            idx
        };
        println!("{idx}");
    }
}
