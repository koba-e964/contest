#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;
/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

template<ll mod = (ll)1e9 + 7>
struct ModInt {
  ll x;
  ModInt(void): x(0) {}
  ModInt(ll x): x(x % mod){}
  ModInt(const ModInt &x): x(x.x) {}
  ModInt operator+(ModInt o) const {
    ll y = x + o.x;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator-(ModInt o) const {
    ll y = x - o.x + mod;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator*(ModInt o) const {
    return ModInt((x * o.x) % mod);
  }
  ll to_ll() const {
    return x;
  }
  bool operator<(ModInt o) const {
    return x < o.x;
  }
  ModInt pow(ll e) {
    assert (e >= 0);
    ModInt sum = 1;
    ModInt cur = *this;
    while (e > 0) {
      if (e % 2) {
        sum = sum * cur;
      }
      cur = cur * cur;
      e /= 2;
    }
    return sum;
  }
  ModInt inv(void) {
    return pow(mod - 2);
  }
};

template<ll mod>
ostream &operator<<(ostream &os, ModInt<mod> mi) {
  return os << mi.x;
}

string s;
int len;
VI tbl;

int n;
ModInt<> a[200000];
ModInt<> coef_gl[200000];

struct node {
  ModInt<> coef, val;
  char ty;
  int idx;
  node *ch[2];
  node(void) {
    ty = '&';
    coef = 1;
    val = 0;
    REP(i, 0, 2) ch[i] = 0;
    idx = -1;
  }
  node(char c): coef(1), val(0), ty(c) {
    REP(i, 0, 2) ch[i] = 0;
    idx = -1;
  }
  void debug(int lv = 0) const {
    REP(i, 0, lv)cerr<<" ";
    cerr << ty << "[" << val << "](" << coef << ")" << endl;
    REP(i, 0, 2) {
      if (ch[i] != NULL) {
        ch[i]->debug(lv + 1);
      }
    }
  }
  void calc(void) {
    if (ty == 'a') {
      assert (idx >= 0);
      val = a[idx];
      return;
    }
    REP(i, 0, 2) ch[i]->calc();
    if (ty == '+') {
      val = ch[0]->val + ch[1]->val;
      return;
    }
    if (ty == '-') {
      val = ch[0]->val - ch[1]->val;
      return;
    }
    assert (ty == '*');
    val = ch[0]->val * ch[1]->val;
    return;
  }
  void dfs(ModInt<> cc) {
    if (ty == 'a') {
      coef = cc;
      coef_gl[idx] = cc;
      return;
    }
    if (ty == '+') {
      coef = cc;
      REP(i, 0, 2) ch[i]->dfs(cc);
      return;
    }
    if (ty == '-') {
      coef = cc;
      ch[0]->dfs(cc);
      ch[1]->dfs(ModInt<>(0)-cc);
      return;
    }
    assert (ty == '*');
    ch[0]->dfs(cc*ch[1]->val);
    ch[1]->dfs(cc*ch[0]->val);
    return;
  }
};

pair<node*, int> parse_term(int pos);
pair<node*, int> parse_value(int pos);
pair<node*, int> parse_expr(int pos) {
  DEBUGP(pos);
  DEBUGP(s.substr(0, pos));
  if (pos == 0) {
    assert(!"empty");
  }
  pair<node*, int> res = parse_term(pos);
  char nxt = pos - res.second > 0 ? s[pos - res.second - 1] : '_';
  if (nxt == '+' || nxt == '-') {
    pair<node*, int> sub2 = parse_expr(pos - res.second - 1);
    node *ret = new node(nxt);
    ret->ch[0] = sub2.first;
    ret->ch[1] = res.first;
    return make_pair(ret, sub2.second + 1 + res.second);
  }
  return res;
}
pair<node*, int> parse_term(int pos) {
  if (pos == 0) {
    assert(!"empty");
  }
  pair<node*, int> res = parse_value(pos);
  char nxt = pos - res.second > 0 ? s[pos - res.second - 1] : '_';
  if (nxt == '*') {
    pair<node*, int> sub2 = parse_term(pos - res.second - 1);
    node *ret = new node(nxt);
    ret->ch[0] = sub2.first;
    ret->ch[1] = res.first;
    return make_pair(ret, sub2.second + 1 + res.second);
  }
  return res;
}
pair<node*, int> parse_value(int pos) {
  if (pos == 0) {
    assert(!"empty");
  }
  char st = s[pos - 1];
  if (st == ')') {
    pair<node*, int> res = parse_expr(pos - 1);
    return make_pair(res.first, res.second + 2);
  }
  assert (st == 'a');
  node *ret = new node('a');
  ret->idx = tbl[pos - 1];
  return make_pair(ret, 1);
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q;
  cin >> s >> q;
  len = s.length();
  tbl = VI(len, -1);
  n = 0;
  REP(i, 0, s.length()) {
    if (s[i] == 'a') {
      tbl[i] = n;
      n++;
    }
  }
  REP(i, 0, n) {
    ll x;
    cin >> x;
    a[i] = x;
  }
  pair<node*, int> res = parse_expr(len);
  res.first->calc();
  res.first->dfs(1);
  // res.first->debug();
  ModInt<> gt = res.first->val;
  REP(i, 0, q) {
    int b;
    ll x;
    cin >> b >> x;
    b--;
    ModInt<> ans = gt + coef_gl[b] * (ModInt<>(x) - a[b]);
    cout << ans << endl;
  }
}
