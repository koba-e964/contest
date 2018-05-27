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
typedef pair<int, PI> PIPI;

const ll mod = 998244353;
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


const int W = 450;
const int N = W * W;

ll a[N];
ll u[W], v[W], invu[W]; // The real value is u[i] * a[i * W + j] + v[i]
ll sum[W]; // sum before adjustment

void init(void) {
  REP(i, 0, W) u[i] = invu[i] = 1;
}

ll query_naive(int l, int r) {
  int rb = l / W;
  ll tot = 0;
  REP(i, l, r) add(tot, a[i]);
  tot = (tot * u[rb] + v[rb] * (r - l)) % mod;
  return tot;
}

ll query(int l, int r) {
  int lb = (l + W - 1) / W;
  int rb = r / W;
  if (lb > rb) {
    return query_naive(l, r);
  }
  ll tot = query_naive(l, W * lb);
  add(tot, query_naive(W * rb, r));
  REP(i, lb, rb) {
    add(tot, sum[i] * u[i] % mod + v[i] * W % mod);
  }
  return tot;
}

void update_mul_naive(int l, int r, ll k, ll cons) {
  int rb = l / W;
  REP(i, l, r) {
    ll nv = a[i] * k % mod;
    add(nv, cons * invu[rb]);
    add(sum[rb], nv - a[i] + mod);
    a[i] = nv;
  }
}
void update_mul(int l, int r, ll k, ll invk, ll cons) {
  int lb = (l + W - 1) / W;
  int rb = r / W;
  if (lb > rb) {
    return update_mul_naive(l, r, k, cons);
  }
  update_mul_naive(l, W * lb, k, cons);
  update_mul_naive(W * rb, r, k, cons);
  REP(i, lb, rb) {
    u[i] = u[i] * k % mod;
    invu[i] = invu[i] * invk % mod;
    v[i] = (v[i] * k + cons) % mod;
  }
}

#if 0
int main(void) {
  init();
  while (1) {
    int l, r, x, y;
    cin >> l >> r >> x >> y;
    if (l == -2) break;
    if (l == -1) {
      cout << query(r, x) << endl;
      continue;
    }
    update_mul(l, r, x, powmod(x, mod - 2), y);
    cerr << "a:";
    REP(i, 0, N) {
      cerr << " " << a[i];
    }
    cerr << endl;
    cerr << "(u, v):";
    REP(i, 0, W) {
      cerr << " (" << u[i] << "," << v[i] << ")";
    }
    cerr << endl;
    cerr << "sum:";
    REP(i, 0, W) {
      cerr << " " << sum[i];
    }
    cerr << endl;
  }
}

#else

vector<PIPI> tbl[N];

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  vector<PIPI> que;
  REP(i, 0, q) {
    int ty;
    cin >> ty;
    if (ty == 1) {
      int l, r, x;
      cin >> l >> r >> x;
      l--;
      tbl[x].push_back(PIPI(i, PI(l, r)));
    } else {
      int l, r;
      cin >> l >> r;
      l--;
      que.push_back(PIPI(i, PI(l, r)));
    }
  }
  REP(x, 1, n + 1) {
    sort(tbl[x].begin(), tbl[x].end());
    map<int, int> rgt, lft;
    REP(i, 0, tbl[x].size()) {
      PIPI t = tbl[x][i];
      int idx = t.first;
      int l = t.second.first;
      int r = t.second.second;
      auto it = rgt.upper_bound(r);
    }
  }
  
}
#endif
