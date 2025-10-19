fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let q = getline().trim().parse::<i32>().unwrap();
    let mut v = vec![0];
    for _ in 0..q {
        let toks = getline().trim().split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let last = v[v.len() - 1];
        if toks[0] == "1" {
            let nxt = if last < 0 {
                last
            } else if toks[1] == "(" {
                last + 1
            } else {
                last - 1
            };
            v.push(nxt);
        } else {
            v.pop();
        }
        println!("{}", if v[v.len() - 1] == 0 {
            "Yes"
        } else {
            "No"
        });
    }
}
