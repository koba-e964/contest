#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

const int DEBUG = 0;
#define DEBUGP(aa) cerr << #aa << "=" << aa << endl;


ll ceil_div(ll a, ll b) {
  ll r = a % b;
  if (r < 0) r += b;
  return (a - r) / b;
}

const int N = 210000;
int bit[N];
void clear(void) {
  REP(i, 0, N) bit[i] = 0;
}
void add(int i, int v) {
  assert (i != 0);
  while (i < N) {
    bit[i] += v;
    i += i & - i;
  }
}

int accum(int i) {
  int sum = 0;
  while (i > 0) {
    sum += bit[i];
    i &= i - 1;
  }
  return sum;
}

ll ca[N];
int ca_size;


// Returns #{(i, j) | i <= j, a[i] * b[j] <= v} >= k
bool check(const VL &a, const VL &b, ll v, ll k) {
  if (DEBUG) DEBUGP(v);
  int n = a.size();
  int m = b.size();
  clear();
  ll count = 0;
  int sz = 0;
  REP(i, 0, m) {
    if (i < n) {
      int idx = a[i];
      add(idx + 1, 1);
      sz++;
    }
    if (DEBUG)
      DEBUGP(b[i]);
    if (b[i] == 0) {
      if (v >= 0) count += sz;
    } else if (b[i] > 0) {
      ll lim = ceil_div(v, b[i]);
      int idx = upper_bound(ca, ca + ca_size, lim) - ca;
      int cc = accum(idx);
      if (DEBUG) {
        DEBUGP("pos");
        DEBUGP(lim);
        DEBUGP(cc);
      }
      count += cc;
    } else {
      ll lim = ceil_div(-v - b[i] - 1, -b[i]);
      int idx = lower_bound(ca, ca + ca_size, lim) - ca;
      int cc = sz - accum(idx);
      if (DEBUG) {
        DEBUGP("neg");
        DEBUGP(lim);
        DEBUGP(cc);
      }
      count += cc;
    }
    if (DEBUG) {
      DEBUGP(count);
    }
  }
  if (DEBUG) cerr << endl;
  return count >= k;
}

const ll bias = 1e11;

int main(void) {
  int n, m, x;
  ll k;
  scanf("%d%d%d%lld", &n, &m, &x, &k);
  VL a(n);
  REP(i, 0, n) scanf("%lld", &a[i]);
  VL b(m);
  REP(i, 0, m) scanf("%lld", &b[i]);
  VL nb(m - x);
  REP(i, 0, m - x) nb[i] = b[i + x];
  REP(i, 0, n) ca[i] = a[i];
  sort(ca, ca + n);
  ca_size = unique(ca, ca + n) - ca;
  REP(i, 0, n) a[i] = lower_bound(ca, ca + ca_size, a[i]) - ca;
  ll pass = 5e10, fail = -5e10;
  while (pass - fail > 1) {
    ll mid = ((pass + fail + 2 * bias) >> 1) - bias;
    if (check(a, nb, mid, k)) pass = mid;
    else fail = mid;
  }
  printf("%lld\n", pass);
}
