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
    return ModInt(y, 0);
  }
  ModInt operator-(ModInt o) const {
    ll y = x - o.x + mod;
    if (y >= mod) y -= mod;
    return ModInt(y, 0);
  }
  ModInt operator*(ModInt o) const {
    return ModInt((x * o.x) % mod, 0);
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
private:
  ModInt(ll x, int bypass): x(x) {
  }
};

template<ll mod>
ostream &operator<<(ostream &os, ModInt<mod> mi) {
  return os << mi.x;
}

const int W = 450;
const int N = W * W;
const int M = 20;

int cell[N][M];
ModInt<> hsh[W][M];
int targ[W][M];
ModInt<> base, wbase;

void recalc(int blk, int x) {
  int a = targ[blk][x];
  ModInt<> sum(0), cur(1);
  REP(i, blk * W, blk * W + W) {
    sum = sum + ModInt<>(cell[i][a]) * cur;
    cur = cur * base;
  }
  hsh[blk][a] = sum;
}

void swap_naive(int x, int y, int l, int r) {
  // l, r are in the same block
  int blk = l / W;
  int a = targ[blk][x], b = targ[blk][y];
  REP(i, l, r) {
    swap(cell[i][a], cell[i][b]);
  }
  recalc(blk, x);
  recalc(blk, y);
}

ModInt<> calc_naive(int x, int l, int r) {
  int blk = l / W;
  int a = targ[blk][x];
  ModInt<> sum(0), cur(1);
  REP(i, l, r) {
    sum = sum + ModInt<>(cell[i][a]) * cur;
    cur = cur * base;
  }
  return sum;
}
ModInt<> calc_smart(int x, int blk) {
  int a = targ[blk][x];
  return hsh[blk][a];
}

void init(void) {
  base = ModInt<>(1000000);
  wbase = base.pow(W);
  REP(i, 0, W) {
    REP(j, 0, M) {
      hsh[i][j] = 0;
      targ[i][j] = j;
    }
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init();
  int n, m;
  cin >> n >> m;
  vector<string> s(m);
  REP(i, 0, m) cin >> s[i];
  REP(i, 0, n) {
    REP(j, 0, m) {
      cell[i][j] = s[j][i] - 'a' + 1;
    }
  }
  REP(i, 0, W) {
    REP(j, 0, m) {
      recalc(i, j);
    }
  }
  int q;
  cin >> q;
  REP(very_long_name, 0, q) {
    int ty;
    int x, y, l, r;
    cin >> ty >> x >> y >> l >> r;
    x--, y--, l--;
    int lb = (l + W - 1) / W;
    int rb = r / W;
    if (ty == 1) {
      if (lb > rb) {
	swap_naive(x, y, l, r);
      } else {
	swap_naive(x, y, l, W * lb);
	REP(i, lb, rb) {
	  swap(targ[i][x], targ[i][y]);
	}
	swap_naive(x, y, W * rb, r);
      }
    } else {
      ModInt<> ans(0);
      if (lb > rb) {
        ans = calc_naive(x, l, r);
      } else {
	ans = calc_naive(x, l, W * lb);
	int len = W * lb - l;
	ModInt<> cur = base.pow(len);
	REP(i, lb, rb) {
	  ans = ans + calc_smart(x, i) * cur;
	  cur = cur * wbase;
	}
	ans = ans + calc_naive(x, W * rb, r) * cur;
      }
      cout << ans << endl;
    }
  }
}
