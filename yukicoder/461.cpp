#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;
typedef pair<ll, ll> PL;

ll gcd(ll x, ll y) {
  return y == 0 ? x : gcd(y, x % y);
}

PL add(PL x, PL y) {
  PL w = PL(x.first * y.second + x.second * y.first,
		x.second * y.second);
  return w;
}

PL neg(PL x) { return PL(- x.first, x.second); }

bool fr_le(PL x, PL y) {
  return x.first * y.second <= y.first * x.second;
}
bool fr_lt(PL x, PL y) {
  return x.first * y.second < y.first * x.second;
}
int main(void){
  int n;
  cin >> n;
  vector<PL> pool[3];
  REP(i, 0, 3) {
    pool[i].push_back(PL(0, 1));
  }
  REP(i, 0, n) {
    int p, a, b;
    cin >> p >> a >> b;
    int g = __gcd(a, b);
    a /= g;
    b /= g;
    pool[p].push_back(PL(b, a + b));
  }
  if (pool[0].size() > pool[2].size()) {
    swap(pool[0], pool[2]);
  }
  if (pool[1].size() > pool[2].size()) {
    swap(pool[1], pool[2]);
  }
  ll tot = 0;
  sort(pool[2].begin(), pool[2].end(), fr_lt);
  REP(i, 0, pool[0].size()) {
    PL u = pool[0][i];
    REP(j, 0, pool[1].size()) {
      PL v = pool[1][j];
      if (not fr_le(add(u, v), PL(1, 1))) {
	continue;
      }
      PL ma = u;
      if (fr_le(u, v)) { ma = v; }
      // find #{x | x + u <= 1 && x + v <= 1 && x + u + v != 1}
      PL x_ma = add(PL(1, 1), neg(ma));
      int idx = upper_bound(pool[2].begin(), pool[2].end(), x_ma, fr_lt)
	- pool[2].begin();
      tot += idx;
      PL excl = add(PL(1, 1), neg(add(u, v)));
      {
	int idx1 = upper_bound(pool[2].begin(), pool[2].end(), excl, fr_lt)
	- pool[2].begin();
	int idx2 = lower_bound(pool[2].begin(), pool[2].end(), excl, fr_lt)
	- pool[2].begin();
	assert (idx1 - idx2 <= 1);
	assert (idx1 >= idx2);
	tot -= idx1 - idx2;
      }
    }
  }
  cout << tot << endl;
}
