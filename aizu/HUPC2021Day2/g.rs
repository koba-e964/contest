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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

/// Verified by https://atcoder.jp/contests/abc198/submissions/21774342
mod mod_int {
    use std::ops::*;
    pub trait Mod: Copy { fn m() -> i64; }
    #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt<M> { pub x: i64, phantom: ::std::marker::PhantomData<M> }
    impl<M: Mod> ModInt<M> {
        // x >= 0
        pub fn new(x: i64) -> Self { ModInt::new_internal(x % M::m()) }
        fn new_internal(x: i64) -> Self {
            ModInt { x: x, phantom: ::std::marker::PhantomData }
        }
        pub fn pow(self, mut e: i64) -> Self {
            debug_assert!(e >= 0);
            let mut sum = ModInt::new_internal(1);
            let mut cur = self;
            while e > 0 {
                if e % 2 != 0 { sum *= cur; }
                cur *= cur;
                e /= 2;
            }
            sum
        }
        #[allow(dead_code)]
        pub fn inv(self) -> Self { self.pow(M::m() - 2) }
    }
    impl<M: Mod> Default for ModInt<M> {
        fn default() -> Self { Self::new_internal(0) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x + other.x;
            if sum >= M::m() { sum -= M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self { ModInt::new(self.x * other.into().x % M::m()) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
        fn add_assign(&mut self, other: T) { *self = *self + other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
        fn sub_assign(&mut self, other: T) { *self = *self - other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
        fn mul_assign(&mut self, other: T) { *self = *self * other; }
    }
    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        fn neg(self) -> Self { ModInt::new(0) - self }
    }
    impl<M> ::std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M: Mod> ::std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let (mut a, mut b, _) = red(self.x, M::m());
            if b < 0 {
                a = -a;
                b = -b;
            }
            write!(f, "{}/{}", a, b)
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self { Self::new(x) }
    }
    // Finds the simplest fraction x/y congruent to r mod p.
    // The return value (x, y, z) satisfies x = y * r + z * p.
    fn red(r: i64, p: i64) -> (i64, i64, i64) {
        if r.abs() <= 10000 {
            return (r, 1, 0);
        }
        let mut nxt_r = p % r;
        let mut q = p / r;
        if 2 * nxt_r >= r {
            nxt_r -= r;
            q += 1;
        }
        if 2 * nxt_r <= -r {
            nxt_r += r;
            q -= 1;
        }
        let (x, z, y) = red(nxt_r, r);
        (x, y - q * z, z)
    }
} // mod mod_int

macro_rules! define_mod {
    ($struct_name: ident, $modulo: expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// Parsing
fn tag<'a>(s: &'a [u8], t: &[u8]) -> Option<(&'a [u8], ())> {
    if s.len() < t.len() {
        return None;
    }
    if &s[..t.len()] == t {
        Some((&s[t.len()..], ()))
    } else {
        None
    }
}

fn plus_char<F: Fn(u8) -> bool>(s: &[u8], f: F) -> Option<(&[u8], Vec<u8>)> {
    if s.is_empty() {
        return None;
    }
    if !f(s[0]) {
        return None;
    }
    let mut res = vec![s[0]];
    let mut pos = 1;
    while pos < s.len() && f(s[pos]) {
        res.push(s[pos]);
        pos += 1;
    }
    Some((&s[pos..], res))
}

fn plus<T, F: Fn(&[u8]) -> Option<(&[u8], T)>>(s: &[u8], f: F) -> Option<(&[u8], Vec<T>)> {
    if s.is_empty() {
        return None;
    }
    let (mut s, x) = f(s)?;
    let mut res = vec![x];
    while !s.is_empty() {
        if let Some((to, x)) = f(s) {
            res.push(x);
            s = to;
        } else {
            break;
        }
    }
    Some((s, res))
}
fn star<T, F: Fn(&[u8]) -> Option<(&[u8], T)>>(mut s: &[u8], f: F) -> Option<(&[u8], Vec<T>)> {
    let mut res = vec![];
    while !s.is_empty() {
        if let Some((to, x)) = f(s) {
            res.push(x);
            s = to;
        } else {
            break;
        }
    }
    Some((s, res))
}

// <program> ::= ( <declaration> | <assignment> | <print> | <for> )+
#[derive(Debug)]
enum ProgLine {
    Dec(u8),
    Asgn(Asgn),
    Prn(Expr),
    For(ForStmt),
}
fn prog_line(s: &[u8]) -> Option<(&[u8], ProgLine)> {
    if let Some((s, v)) = decl(s) {
        return Some((s, ProgLine::Dec(v)));
    }
    if let Some((s, v)) = asgn(s) {
        return Some((s, ProgLine::Asgn(v)));
    }
    if let Some((s, v)) = print(s) {
        return Some((s, ProgLine::Prn(v)));
    }
    if let Some((s, v)) = for_stmt(s) {
        return Some((s, ProgLine::For(v)));
    }
    None
}
// <declaration> ::= <value_name><EOL>
fn decl(s: &[u8]) -> Option<(&[u8], u8)> {
    let (s, name) = value_name(s)?;
    if s.is_empty() {
        return Some((s, name));
    }
    None
}

// <assignment> ::= <value_name> "<-" <expression><EOL>
#[derive(Debug)]
struct Asgn {
    name: u8,
    e: Expr,
}
fn asgn(s: &[u8]) -> Option<(&[u8], Asgn)> {
    let (s, name) = value_name(s)?;
    let (s, _) = tag(s, b"<-")?;
    let (s, e) = expr(s)?;
    if !s.is_empty() {
        return None;
    }
    Some((s, Asgn {
        name: name,
        e: e,
    }))
}

// <print> ::= "print(" <expression> ")" <EOL>
fn print(s: &[u8]) -> Option<(&[u8], Expr)> {
    let (s, _) = tag(s, b"print(")?;
    let (s, x) = expr(s)?;
    let (s, _) = tag(s, b")")?;
    if !s.is_empty() {
        return None;
    }
    Some((s, x))
}
// <for> ::= <for_element>+( <assignment> | <print> )
#[derive(Debug)]
enum ForStmt {
    Asgn(Vec<ForElem>, Asgn),
    Prn(Vec<ForElem>, Expr),
}
fn for_stmt(s: &[u8]) -> Option<(&[u8], ForStmt)> {
    let (s, l) = plus(s, for_elem)?;
    if let Some((s, asgn)) = asgn(s) {
        return Some((s, ForStmt::Asgn(l, asgn)));
    }
    let (s, p) = print(s)?;
    Some((s, ForStmt::Prn(l, p)))
}
// <for_element> ::= "for(" <expression> ")"
#[derive(Debug)]
struct ForElem {
    e: Expr,
}

fn for_elem(s: &[u8]) -> Option<(&[u8], ForElem)> {
    let (s, _) = tag(s, b"for(")?;
    let (s, x) = expr(s)?;
    let (s, _) = tag(s, b")")?;
    Some((s, ForElem {
        e: x,
    }))
}

// <expression> ::= <term1><term2>*
#[derive(Debug)]
struct Expr {
    coef: HashMap<u8, MInt>,
    con: MInt,
}
fn expr(s: &[u8]) -> Option<(&[u8], Expr)> {
    let (s, mut t1) = term1(s)?;
    let (s, t2) = star(s, term2)?;
    for e in t2 {
        for (k, v) in e.coef {
            *t1.coef.entry(k).or_insert(MInt::new(0)) += v;
        }
        t1.con += e.con;
    }
    Some((s, t1))
}

// <term1> ::= "-"?( <number> | <value_name> )
fn term1(mut s: &[u8]) -> Option<(&[u8], Expr)> {
    if s.is_empty() {
        return None;
    }
    let mut neg = false;
    if s[0] == b'-' {
        s = &s[1..];
        neg = true;
    }
    if let Some((s, mut v)) = number(s) {
        if neg {
            v = -v;
        }
        return Some((s, Expr {
            coef: HashMap::default(),
            con: v,
        }));
    }
    let (s, name) = value_name(s)?;
    let c = if neg {
        MInt::new(MOD - 1)
    } else {
        MInt::new(1)
    };
    Some((s, Expr {
        coef: vec![(name, c)].into_iter().collect(),
        con: MInt::new(0),
    }))
}
// <term2> ::= ( "+" | "-" )( <number> | <value_name> )
fn term2(s: &[u8]) -> Option<(&[u8], Expr)> {
    if s.is_empty() {
        return None;
    }
    if s[0] == b'+' {
        return term1(&s[1..]);
    }
    term1(s)
}
// <number> ::= 非負整数（10進数表記で、先頭に余分な "+" や "0" はない）
fn number(s: &[u8]) -> Option<(&[u8], MInt)> {
    let (s, t) = number_like(s)?;
    let mut x = MInt::new(0);
    for d in t {
        x = x * 10 + (d - b'0') as i64;
    }
    Some((s, x))
}

fn number_like(s: &[u8]) -> Option<(&[u8], Vec<u8>)> {
    plus_char(s, |c| b'0' <= c && c <= b'9')
}

// <value_name> ::= 1文字の英小文字
fn value_name(s: &[u8]) -> Option<(&[u8], u8)> {
    if s.is_empty() {
        return None;
    }
    if s[0] >= b'a' && s[0] <= b'z' {
        Some((&s[1..], s[0]))
    } else {
        None
    }
}

fn parse_all(s: &[Vec<u8>]) -> Vec<ProgLine> {
    let mut res = vec![];
    for s in s {
        let (s, x) = prog_line(s).unwrap();
        assert!(s.is_empty());
        res.push(x);
    }
    res
}

fn eval(env: &HashMap<u8, MInt>, e: &Expr) -> MInt {
    let mut v = e.con;
    for (&k, &coef) in &e.coef {
        v += env[&k] * coef;
    }
    v
}

fn execute(s: Vec<ProgLine>) -> Vec<(i64, MInt)> {
    let mut ans = vec![];
    let mut hm = HashMap::new();
    for s in &s {
        match s {
            ProgLine::Dec(name) => {
                hm.insert(*name, MInt::new(0));
            }
            ProgLine::Asgn(asgn) => {
                hm.insert(asgn.name, eval(&hm, &asgn.e));
            }
            ProgLine::Prn(e) => {
                ans.push((1, eval(&hm, e)));
            }
            ProgLine::For(ForStmt::Asgn(l, asgn)) => {
                let mut iter = 1i64;
                for elem in l {
                    let x = eval(&hm, &elem.e);
                    iter *= x.x;
                }
                let mut env = hm.clone();
                env.insert(asgn.name, 0.into());
                let con = eval(&env, &asgn.e);
                let &init = hm.get(&asgn.name).unwrap_or(&0.into());
                let &fb = asgn.e.coef.get(&asgn.name).unwrap_or(&0.into());
                let v;
                if fb == 1.into() {
                    v = init + con * iter;
                } else {
                    let stab = -con * (fb - 1).inv();
                    let tmp = (init - stab) * fb.pow(iter);
                    v = tmp + stab;
                }
                hm.insert(asgn.name, v);
            }
            ProgLine::For(ForStmt::Prn(l, e)) => {
                let mut iter = 1i64;
                for elem in l {
                    let x = eval(&hm, &elem.e);
                    iter *= x.x;
                }
                ans.push((iter, eval(&hm, e)));
            }
        }
    }
    ans
}

fn normalize<T: PartialEq>(x: Vec<(i64, T)>) -> Vec<(i64, T)> {
    let mut ans: Vec<(i64, T)> = vec![];
    for (a, t) in x {
        if a == 0 { continue; }
        if ans.len() > 0 && ans[ans.len() - 1].1 == t {
            let l = ans.len() - 1;
            ans[l].0 += a
        } else {
            ans.push((a, t));
        }
    }
    ans
}

fn calc(s: &[Vec<u8>], t: &[Vec<u8>]) -> bool {
    let s = execute(parse_all(s));
    let t = execute(parse_all(t));
    normalize(s) == normalize(t)
}

// Tags: parsing
fn main() {
    loop {
        let n: usize = get();
        let m: usize = get();
        if (n, m) == (0, 0) { return; }
        let s: Vec<Vec<_>> =
            (0..n).map(|_| get_word().bytes().collect()).collect();
        let t: Vec<Vec<_>> =
            (0..m).map(|_| get_word().bytes().collect()).collect();
        println!("{}", if calc(&s, &t) { "Yes" } else { "No" });
    }
}
