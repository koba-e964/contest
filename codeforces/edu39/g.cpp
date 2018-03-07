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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

template<class T>
void vec_debug(const vector<T> &v) {
  REP(i, 0, v.size())
    cerr << " " << v[i];
  cerr << endl;
}

const int N = 411000;
int bit[N];

void add(int x, int v) {
  while (x < N) {
    bit[x] = max(bit[x], v);
    x += x & -x;
  }
}

int accum(int x) {
  int tot = 0;
  while (x > 0) {
    tot = max(tot, bit[x]);
    x &= x - 1;
  }
  return tot;
}

VI lis(const VL &a) {
  int n = a.size();
  VI m(n + 1);
  VI dp(n);
  int maxl = 0;
  REP(i, 0, n) {
    int pass = 0, fail = maxl + 1;
    while (fail - pass > 1) {
      int mid = (fail + pass) / 2;
      if (a[m[mid]] <= a[i]) {
	pass = mid;
      } else {
	fail = mid;
      }
    }
    dp[i] = pass + 1;
    maxl = max(maxl, pass + 1);
    m[pass + 1] = i;
  }
  if (DEBUG) {
    cerr << "a:";
    REP(i, 0, n) cerr << " " << a[i];
    cerr << endl;
    cerr << "dp:";
    REP(i, 0, n) cerr << " " << dp[i];
    cerr << endl;
  }
  return dp;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i]-=i;
  }
  VL b(n);
  REP(i, 0, n) {
    b[i] = -a[n - 1 - i];
  }
  VI dpl = lis(a);
  VI dpr = lis(b);
  reverse(dpr.begin(), dpr.end());
  VL occ(2 * n);
  REP(i, 0, n) {
    occ[2 * i] = a[i];
    occ[2 * i + 1] = a[i] + 1;
  }
  sort(occ.begin(), occ.end());
  occ.erase(unique(occ.begin(), occ.end()), occ.end());
  map<ll, int> inv;
  REP(i, 0, occ.size()) inv[occ[i]] = i;
  VI dec(n);
  REP(i, 0, n) {
    dec[i] = inv[a[i] + 1] + 1;
    a[i] = inv[a[i]] + 1;
  }
  if (DEBUG) {
    vec_debug(a);
    vec_debug(dec);
  }
  int ma = 0;
  REP(i, 0, n) {
    if (i < n - 1) {
      ma = max(ma, accum(dec[i + 1]) + dpr[i + 1]);
    }
    ma = max(ma, dpl[i]);
    add(a[i], dpl[i]);
  }
  cout << max(0, n - 1 - ma) << "\n";
}
