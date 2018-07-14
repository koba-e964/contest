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

#define MOCK 0

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 100100;
int fac[N];

void init(void) {
  REP(i, 2, N) {
    if (fac[i] != 0) continue;
    fac[i] = i;
    for (int j = 2; j * i < N; ++j) {
      fac[i * j] = i;
    }
  }
}

map<int, int> div(int x) {
  map<int, int> tap;
  while (x > 1) {
    int f = fac[x];
    int e = 0;
    while (x % f == 0) {
      x /= f;
      e++;
    }
    tap[f] = e;
  }
  return tap;
}

ll ris(const VI &fac) {
  int x = 1;
  for (int e: fac) {
    x *= e + 1;
  }
  return x;
}
ll iroha(const VI &f1, const VI &f2) {
  ll x = 1;
  REP(i, 0, f1.size()) {
    x *= min(f1[i], f2[i]) + 1;
  }
  return x;
}

ll meguru(const VI &f1, const VI &f2, const VI &f3, bool debug) {
  ll ans = 0;
  int n = f1.size();
  // a = b = c
  ll x = 1;
  REP(i, 0, n) {
    int e = min(min(f1[i], f2[i]), f3[i]);
    x *= e + 1;
  }
  ans += x;
  if (debug) DEBUGP(ans);
  // a = b < c
  x = 1;
  ll y = 0;
  REP(i, 0, n) {
    int e = min(f1[i], f2[i]);
    ll u = 0;
    REP(j, 0, e + 1) {
      u += max(0, f3[i] - j);
    }
    REP(j, i + 1, n) {
      u *= f3[j] + 1;
      u *= min(f1[j], f2[j]) + 1;
    }
    REP(j, 0, i) {
      u *= min(min(f1[j], f2[j]), f3[j]) + 1;
    }
    y += u;
  }
  ans += x * y;
  if (debug) DEBUGP(ans);
  // a < b = c
  x = 1;
  y = 0;
  REP(i, 0, n) {
    int e = min(f2[i], f3[i]);
    ll u = 0;
    REP(j, 0, e + 1) {
      u += min(f1[i] + 1, j);
    }
    REP(j, i + 1, n) {
      u *= f1[j] + 1;
    }
    y += u;
  }
  ans += x * y;
  if (debug) DEBUGP(ans);
  // a < b < c
  REP(i, 0, n) {
    REP(j, 0, n) {
      ll y = 1;
      REP(k, 0, min(i, j)) {
	y *= min(min(f1[k], f2[k]), f3[k]) + 1;
      }
      if (i == j) {
	
      }
    }
  }
  return ans;
}


VI make(const VI &bv, map<int, int> &d) {
  VI ret(bv.size());
  REP(i, 0, bv.size()) ret[i] = d[bv[i]];
  return ret;
}

int perm[6][3] =
  {
   {0,1,2},
   {0,2,1},
   {1,2,0},
   {1,0,2},
   {2,0,1},
   {2,1,0},
  };

const int inf = 1e7;

ll f(int a, int b, int c) {
  map<int, int> da = div(a), db = div(b), dc = div(c);
  set<int> base;
  for (PI e: da) base.insert(e.first);
  for (PI e: db) base.insert(e.first);
  for (PI e: dc) base.insert(e.first);
  VI bv(base.begin(), base.end());
  sort(bv.begin(), bv.end());
  int n = bv.size();
  VI ma = make(bv, da), mb = make(bv, db), mc = make(bv, dc);
  vector<VI> ms(3);
  ms[0] = ma;
  ms[1] = mb;
  ms[2] = mc;
  VL mess(64);
  REP(bits, 1, 64) {
    VI ta(n, inf), tb(n, inf), tc(n, inf);
    REP(i, 0, 6) {
      if ((bits & 1 << i) == 0) continue;
      REP(j, 0, n) {
	ta[j] = min(ta[j], ms[perm[i][0]][j]);
	tb[j] = min(tb[j], ms[perm[i][1]][j]);
	tc[j] = min(tc[j], ms[perm[i][2]][j]);
      }
    }
    ll x = meguru(ta, tb, tc, bits == 12);
    if (bits == 12) {
      REP(i, 0, n) {
	cerr << ta[i] << " " << tb[i] << " " << tc[i] << endl;
      }
      cerr << "x = " << x << endl;
    }
    mess[bits] = x;
  }
  for (int bits = 63; bits >= 1; --bits) {
    for (int sup = bits + 1; sup < 64; ++sup) {
      if ((sup & bits) != bits) continue;
      mess[bits] -= mess[sup];
    }
  }
  VL tt(7);
  ll tot = 0;
  REP(bits, 1, 64) {
    int bc = __builtin_popcount(bits);
    if (bc <= 2) {
      cerr << bits << " => " << mess[bits] << endl;
    }
    tt[bc] += mess[bits];
  }
  if (1) {
    REP(i, 1, 7) {
      DEBUGP(i);
      DEBUGP(tt[i]);
    }
  }
  REP(i, 1, 7) tot += tt[i];
  return tot;
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    int a, b, c;
#if MOCK
    int kk = 83160;
    a = b = c = kk;
#else
    cin >> a >> b >> c;
#endif
    cout << f(a, b, c) << "\n";
  }
}
