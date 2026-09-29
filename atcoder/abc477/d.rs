fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, q] = ints[..] else { panic!() };
    let mut ops = vec![];
    let mut covered = vec![false; n];
    for _ in 0..q {
        let vals = getline().trim().split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let ty = vals[0].clone();
        let x = vals[1].clone();
        if ty == "1" {
            let x = x.parse::<usize>().unwrap() - 1;
            ops.push(Ok(x));
            covered[x] ^= true;
        } else {
            let c = x.chars().next().unwrap();
            ops.push(Err(c));
        }
    }
    let mut done = vec!['+'; n];
    let mut rem = std::collections::BTreeSet::new();
    for i in 0..n {
        if !covered[i] {
            rem.insert(i);
        }
    }
    for qs in ops.into_iter().rev() {
        match qs {
            Ok(x) => {
                if done[x] == '+' {
                    if rem.contains(&x) {
                        rem.remove(&x);
                    } else {
                        rem.insert(x);
                    }
                }
            }
            Err(c) => {
                for &v in &rem {
                    done[v] = c;
                }
                rem.clear();
            }
        }
    }
    for i in 0..n {
        print!("{}", if done[i] == '+' { 'a' } else { done[i] });
    }
    println!();
}
