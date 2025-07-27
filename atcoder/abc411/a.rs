fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let p = getline().trim().to_string();
    let l: usize = getline().trim().parse().unwrap();
    println!("{}", if p.len() >= l {
        "Yes"
    } else {
        "No"
    });
}
