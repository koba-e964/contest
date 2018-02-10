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
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;

const int N = 18;
ll x[N],c[N],v[N];
ll acc[N+1];

ll cb[1 << N], vb[1 << N];


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> x[i];
  }
  REP(i, 0, n) {
    cin >> c[i];
  }
  REP(i, 0, n) {
    cin >> v[i];
  }
  acc[0] = 0;
  REP(i, 0, n) {
    acc[i + 1] = acc[i] + x[i];
  }
  vector<PL> pool(1 << n);
  REP(bits, 0, 1 << n) {
    ll cost = 0;
    ll val = 0;
    REP(i, 0, n) {
      if (bits & 1 << i) {
	cost += c[i];
	val += v[i];
      }
    }
    pool[bits] = PL(cost, val);
    cb[bits] = cost;
    vb[bits] = val;
  }
  sort(pool.begin(), pool.end());
  REP(i, 1, 1 << n) {
    pool[i].second = max(pool[i].second, pool[i - 1].second);
  }
  VL dp(1 << n);
  dp[0] = 0;
  REP(bits, 0, 1 << n) {
    ll mi = 1e18;
    if (bits == 0) {
      mi = 0;
    } else {
      REP(i, 0, n) {
	if ((bits & 1 << i) == 0) continue;
	int nb = bits ^ 1 << i;
	mi = min(mi, dp[nb]);
      }
    }
    ll ma = 0;
    int bc = __builtin_popcount(bits);
    ll mon = acc[n - bc];
    int full = bits;
    int sub = full;
    if(0) {
      DEBUGP(mon);
      DEBUGP(full);
    }
    while (1) {
      if (cb[sub] <= mon) {
	ma = max(ma, vb[sub]);
      }
      if (sub == 0) {
	break;
      } else {
	sub = (sub - 1) & full;
      }
    }
    dp[bits] = max(ma, mi);
  }
  cout << dp[(1 << n) - 1] << endl;
}
