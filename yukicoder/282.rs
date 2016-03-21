#[allow(unused_imports)]
use std::cmp::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
#[allow(dead_code)]
fn getword() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() ||u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
#[allow(dead_code)]
fn parse<T : std::str::FromStr>(s : &str) -> T {
     return s.parse::<T>().ok().unwrap();
}

fn query(n: usize, ary: &mut [Option<usize>], que: &[(usize, usize)]) {
    let mut output = vec![0; 2 * n];
    let mut numq = 0;
    for i in 0 .. n {
        if i < que.len() {
            let (x, y) = que[i];
            if let (Some(u), Some(v)) = (ary[x], ary[y]) {
                output[2 * i] = u;
                output[2 * i + 1] = v;
                numq += 1;
            }
        }
    }
    if numq == 0 { // no question
        return;
    }
    print!("?");
    for o in output.iter() {
        print!(" {}", o);
    }
    println!("");
    let mut ans = vec!['@'; n];
    for i in 0 .. n {
        ans[i] = getword().chars().nth(0).unwrap();
        if i < n && ans[i] == '>' {
            let (x, y) = que[i];
            //swap
            if let (Some(_), Some(_)) = (ary[x], ary[y]) {
                let tmp = ary[x];
                ary[x] = ary[y];
                ary[y] = tmp;
            }
        }
    }
    
}

fn bitonic_sort(n: usize, exp: usize, ary: &mut [Option<usize>]) {
    assert!(ary.len() == (1 << exp));
    for i in 0 .. exp {
        let s = 1 << i;
        let t = 1 << (exp - 1 - i);
        let mut que = Vec::new();
        for k in 0 .. t {
            for p in 0 .. s {
                que.push((p + 2 * s * k, 2 * s - 1 - p + 2 * s * k));
            }
        }
        query(n, ary, &que);
        for j in (0 .. i).rev() {
            que.clear();
            let s = 1 << j;
            let t = 1 << (exp - 1 - j);
            for k in 0 .. t {
                for p in 0 .. s {
                    que.push((p + 2 * s * k, p + s + 2 * s * k)); 
                }
            }
            query(n, ary, &que);
        }
    }
}

fn main() {
    let n: usize = parse(&getword());
    let mut exp = 1;
    while n > (1 << exp) { // find the least exp s.t. n <= 2^exp.
        exp += 1;
    }
    let mut ary = vec![Some(1usize);1 << exp];
    for i in 0 .. 1 << exp {
        ary[i] = if i < n { Some(i + 1) } else { None };
    }
    bitonic_sort(n, exp, &mut ary); // perform bitonic sort on a 2^exp-elem array.
    print!("!");
    for i in 0 .. n {
        print!(" {}", ary[i].unwrap());
    }
    println!("");
}
