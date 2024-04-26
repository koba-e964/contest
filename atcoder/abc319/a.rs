fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let top = [
        ("tourist", 3858),
        ("ksun48", 3679),
        ("Benq", 3658),
        ("Um_nik", 3648),
        ("apiad", 3638),
        ("Stonefeang", 3630),
        ("ecnerwala", 3613),
        ("mnbvmar", 3555),
        ("newbiedmy", 3516),
        ("semiexp", 3481),
    ];
    let s = getline().trim().to_string();
    for (a, b) in top {
        if a == s {
            println!("{}", b);
        }
    }
}
