fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let targ = [
        "ACE","BDF", "CEG", "DFA", "EGB", "FAC", "GBD",
    ];
    let s = getline().trim().to_owned();
    println!("{}", if targ.into_iter().any(|t| &s == t) { "Yes" } else { "No" });
}
