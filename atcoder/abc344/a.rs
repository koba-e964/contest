fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut bar = vec![];
    for i in 0..n {
        if s[i] == '|' {
            bar.push(i);
        }
    }
    for i in 0..bar[0] {
        print!("{}", s[i]);
    }
    for i in bar[1] + 1..n {
        print!("{}", s[i]);
    }
    println!();
}
