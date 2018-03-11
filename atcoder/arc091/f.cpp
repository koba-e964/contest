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


#if 0

const int N = 200;

int main(void) {
  int k;
  cin >> k;
  VI dp(N);
  dp[0] = 0;
  REP(i, 1, N) {
    set<int> seen;
    REP(j, 1, (i / k) + 1) {
      seen.insert(dp[i - j]);
    }
    int mex = 0;
    while (seen.count(mex)) mex++;
    dp[i] = mex;
  }
  cerr << "nim[" << k << "]:";
  REP(i, 0, N) {
    cerr << " " << dp[i];
  }
  cerr << endl;
}

#else

ll g_two(ll a, ll k) {
  ll c = a / k;
  ll d = a % k;
  while (d != 0 && c != 0) {
    d %= c + 1;
    if (d == 0) return c;
    d -= c + 1;
    d += k;
    c -= 1;
  }
  return c;
}

ll g(ll a, ll k) {
  // DEBUGP(a);
  if (a % k == 0) {
    return a / k;
  }
  if (a < k) {
    return 0;
  }
  if (k >= 40000) {
    return g_two(a, k);
  }
  return g(a - a / k - 1, k);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n), k(n);
  ll gru = 0;
  REP(i, 0, n) {
    cin >> a[i] >> k[i];
    ll val = g(a[i], k[i]);
    if (0) {
      DEBUGP(a[i]);
      DEBUGP(k[i]);
      DEBUGP(val);
    }
    gru ^= val;
  }
  cout << (gru == 0 ? "Aoki" : "Takahashi") << endl;
}

#endif
