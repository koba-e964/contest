use std::collections::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn int_t(int: &HashMap<String, i32>) -> i32 {
    let x = get_word();
    if let Ok(x) = x.parse() {
        x
    } else {
        int[&x]
    }
}

fn int_ex(int: &HashMap<String, i32>) -> i32 {
    let mut x = int_t(int);
    loop {
        let op = get_word();
        if op == ";" {
            return x;
        }
        let y = int_t(int);
        x = if op == "+" { x + y } else { x - y };
    }
}

fn vec_t(int: &HashMap<String, i32>, vec: &HashMap<String, Vec<i32>>) -> Vec<i32> {
    let x = get_word();
    if x == "[" {
        let mut ans = vec![];
        loop {
            let v = int_t(int);
            ans.push(v);
            let sep = get_word();
            if sep == "]" { return ans; }
        }
    } else {
        vec[&x].clone()
    }
}

fn vec_ex(int: &HashMap<String, i32>, vec: &HashMap<String, Vec<i32>>) -> Vec<i32> {
    let mut x = vec_t(int, vec);
    loop {
        let op = get_word();
        if op == ";" {
            return x;
        }
        let y = vec_t(int, vec);
        if op == "+" {
            for i in 0..x.len() {
                x[i] += y[i];
            }
        } else {
            for i in 0..x.len() {
                x[i] -= y[i];
            }
        }
    }
}

fn main() {
    let n: usize = get();
    let mut int = HashMap::new();
    let mut vec = HashMap::new();
    for _ in 0..n {
        let ty = get_word();
        if ty == "int" {
            let var = get_word();
            get_word();
            let x = int_ex(&int);
            int.insert(var, x);
            continue;
        }
        if ty == "vec" {
            let var = get_word();
            get_word();
            let x = vec_ex(&int, &vec);
            vec.insert(var, x);
            continue;
        }
        if ty == "print_int" {
            let x = int_ex(&int);
            println!("{}", x);
            continue;
        }
        let x = vec_ex(&int, &vec);
        print!("[ ");
        for x in x {
            print!("{} ", x);
        }
        println!("]");
    }
}
