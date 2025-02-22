fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut st = vec![];
    for c in getline().trim().chars() {
        match c {
            '>' => {
                if let Some('<') = st.last() {
                    st.pop();
                } else {
                    println!("No");
                    return;
                }
            },
            ')' => {
                if let Some('(') = st.last() {
                    st.pop();
                } else {
                    println!("No");
                    return;
                }
            },
            ']' => {
                if let Some('[') = st.last() {
                    st.pop();
                } else {
                    println!("No");
                    return;
                }
            },
            _ => st.push(c),
        }
    }
    if st.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
