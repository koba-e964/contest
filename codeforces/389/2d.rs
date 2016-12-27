#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
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
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

fn main() {
    let k: usize = get();
    let _n: usize = get();
    let mut memo: HashMap<Vec<u8>, Vec<i32> > = HashMap::new();
    for _ in 0 .. k {
        let s = get_word().into_bytes();
        let v: i32 = get();
        match memo.remove(&s) {
            Some(mut vv) => { vv.push(v); memo.insert(s, vv); },
            None =>  { memo.insert(s, vec![v; 1]); },
        }
    }

    let memo_v: Vec<(Vec<u8>, Vec<i32>)> = memo.clone().into_iter().collect();
    let mut center: i32 = 0;
    let mut tot: i32 = 0;
    for (s, mut vs) in memo_v {
        let mut rev_s = s.clone();
        rev_s.reverse();
        vs.sort();
        vs.reverse();
        if s == rev_s {
            let mut cnt = 0;
            while cnt < vs.len() {
                if cnt == vs.len() - 1 {
                    center = max(center, vs[cnt]);
                } else {
                    let c1 = vs[cnt];
                    let c2 = vs[cnt + 1];
                    if c2 >= 0 {
                        tot += c1 + c2;
                    } else if c1 + c2 <= 0 {
                        center = max(center, c1);
                        break;
                    } else {
                        tot += c1 + c2;
                        center = max(center, -c2);
                        break;
                    }
                }
                cnt += 2;
            }
        } else {
            match memo.remove(&rev_s) {
                None => {},
                Some(mut ww) => {
                    ww.sort();
                    ww.reverse();
                    for i in 0 .. min(vs.len(), ww.len()) {
                        let val = vs[i] + ww[i];
                        if val >= 0 {
                            tot += val;
                        }
                    }
                },
            }
            memo.remove(&s);
        }
    }
    println!("{}", tot + center);
}
