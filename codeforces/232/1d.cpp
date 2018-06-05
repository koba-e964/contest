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

ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}
void add(ll &x, ll y) {
  x = (x + y) % mod;
}


const int N = 1000010;
int p[N];

namespace rangebit {
  ll bit0[N], bit1[N];
  // [1, x]
  void range_add(int x, ll v) {
    int i = x;
    while (i < N) {
      add(bit0[i], v * x);
      add(bit1[i], mod - v);
      i += i & -i;
    }
    i = 1;
    while (i < N) {
      add(bit1[i], v);
      i += i & -i;
    }
  }

  // [1, x]
  ll query(int x) {
    ll tmp = 0;
    int i = x;
    while (i > 0) {
      add(tmp, bit1[i]);
      i &= i - 1;
    }
    tmp = tmp * x % mod;
    i = x;
    while (i > 0) {
      add(tmp, bit0[i]);
      i &= i - 1;
    }
    return tmp;
  }
  void reset(void) {
    REP(i, 0, N) bit0[i] = 0;
    REP(i, 0, N) bit1[i] = 0;
  }
}

namespace bit {
  ll bit[N];
  void add(int x, ll v) {
    while (x < N) {
      ::add(bit[x], v);
      x += x & -x;
    }
  }
  ll query(int x) {
    ll tmp = 0;
    while (x > 0) {
      ::add(tmp, bit[x]);
      x &= x - 1;
    }
    return tmp;
  }
  void reset(void) {
    REP(i, 0, N) bit[i] = 0;
  }
}

#if 0
int main(void) {
  while (1) {
    int ty, x;
    cin >> ty >> x;
    if (ty == -1) break;
    if (ty == 1) {
      range_add(x, 1);
    } else {
      cout << query(x) << endl;
    }
    REP(i, 1, 9) {
      cerr << " " << query(i) - query(i - 1);
    }
    cerr << endl;
  }
}
#else

ll a0[N], a1[N], a2[N], a3[N], a4[N];

ll solve(int n) {
  bit::reset();
  rangebit::reset();
  ll tot = 0;
  ll invc = 0;
  {
    ll cur = 0;
    REP(i, 0, n) {
      ll val = bit::query(p[i] + 1);
      invc += i - val;
      a0[i] = invc % mod;
      a2[i] = p[i] - val;
      a1[i] = p[i] > 0 ? rangebit::query(p[i]) : 0;
      rangebit::range_add(p[i] + 1, 1);
      a4[i] = cur;
      add(cur, mod - i + val);
      add(cur, p[i] - val);
      ll u = bit::query(p[i]);
      // i + (i - 1) + ... + (i - u + 1)
      ll tmp = (i + i - u + 1) * u / 2;
      tmp %= mod;
      add(a1[i], mod - tmp);
      bit::add(p[i] + 1, 1);
    }
  }
  bit::reset();
  REP(i, 0, n) {
    bit::add(i + 1, i);
  }
  REP(i, 0, n) {
    a3[i] = (mod - i) * a2[i] % mod;
    add(a3[i], bit::query(p[i]));
    bit::add(p[i] + 1, mod - p[i]);
  }
  if (0) {
    cerr << "a0:";
    REP(i, 0, n) cerr << " " << a0[i];
    cerr << endl;
    cerr << "a1:";
    REP(i, 0, n) cerr << " " << a1[i];
    cerr << endl;
    cerr << "a2:";
    REP(i, 0, n) cerr << " " << a2[i];
    cerr << endl;
    cerr << "a3:";
    REP(i, 0, n) cerr << " " << a3[i];
    cerr << endl;
    cerr << "a4:";
    REP(i, 0, n) cerr << " " << a4[i];
    cerr << endl;
  }
  ll cur = 1;
  for (int i = n - 1; i >= 0; --i) {
    ll tmp = cur;
    tmp = tmp * (n - i - 1) % mod;
    tmp = tmp * (n - i - 2) % mod;
    tmp = tmp * ((mod + 1) / 4) % mod;
    add(tot, tmp * a2[i]);
    add(tot, ((((i > 0 ? a0[i - 1] : 0) + a4[i]) * a2[i] + a1[i] + a3[i]) % mod) * cur);
    cur = cur * (n - i) % mod;
  }
  add(tot, invc);
  if (n < 6) {
    VI ext(n);
    REP(i, 0, n) ext[i] = i;
    VI pp(p, p + n);
    ll gtruth = 0;
    do {
      if (ext <= pp) {
        REP(i, 0, n) {
          REP(j, i + 1, n) {
            if (ext[i] > ext[j]) gtruth++;
          }
        }
      }
    } while (next_permutation(ext.begin(), ext.end()));
    if (gtruth != tot) {
      REP(i, 0, n) cerr << " " << p[i];
      cerr << endl;
      DEBUGP(gtruth);
    }
  }
  return tot;
}

int main(void) {
  int n;
  scanf("%d", &n);
  REP(i, 0, n) {
    scanf("%d", &p[i]);
    p[i]--;
  }
  printf("%lld\n", solve(n));
  /*
  REP(i, 0, n) p[i] = i;
  do {
    solve(n);
  } while (next_permutation(p, p + n));
  */
}
#endif
