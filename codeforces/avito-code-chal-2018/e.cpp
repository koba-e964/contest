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

#define MOCK 0

const int DEBUG = 0;

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<PI, int> PPII;
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


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
#if MOCK
  cin >> n;
  q = n;
#else
  cin >> n >> q;
#endif
  vector<PPII> pool;
  REP(i, 0, q) {
    int l, r, x;
#if MOCK
    l = 0, r = n; x = i;
#else
    cin >> l >> r >> x;
    l--;
#endif
    pool.push_back(PPII(PI(r, -1), x)); // deletion-first policy
    pool.push_back(PPII(PI(l, 1), x));
  }
  sort(pool.begin(), pool.end());
  VL dp(n + 1);
  dp[0] = 1;
  VI real(n + 1);
  REP(i, 0, pool.size()) {
    PI tt = pool[i].first;
    int kind = tt.second;
    int x = pool[i].second;
    if (kind == 1) {
      for (int i = n - x; i >= 0; --i) {
        add(dp[i + x], dp[i]);
      }
    } else {
      REP(i, 0, n - x + 1) {
        add(dp[i + x], mod - dp[i]);
      }
    }
    REP(i, 0, n + 1) {
      if (dp[i] != 0) real[i] = 1;
    }
    if (DEBUG) {
      REP(i, 0, n + 1) {
        cerr << " " << dp[i];
      }
      cerr << endl;
    }
  }
  VI ans;
  REP(i, 1, n + 1) {
    if (real[i]) ans.push_back(i);
  }
  cout << ans.size() << "\n";
#if !MOCK
  REP(i, 0, ans.size()) {
    cout << ans[i] << (i == (int) ans.size() - 1 ? "\n" : " ");
  }
#endif
}
