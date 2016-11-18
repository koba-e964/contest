#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 200100;
int n;
ll t[N], d[N];

int get_pos(ll time) {
  int idx = upper_bound(t, t + n, time) - t;
  return idx;
}

int main(void){
  ll k;
  cin >> n >> k;
  REP(i, 0, n) {
    cin >> t[i] >> d[i];
  }
  ll lo = -1, hi = 2e9;
  while (hi - lo > 1) {
    ll mid = (hi + lo) / 2;
    VL overfull;
    REP(i, 0, n) {
      if (d[i] > mid) {
	overfull.push_back(t[i]);
      }
    }
    // Yuki has to do jobs in overfull.
    bool ok = 1;
    REP(i, 0, overfull.size() - 1) {
      ok &= overfull[i + 1] - overfull[i] >= k;
    }
    if (ok) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cout << hi << endl;
  typedef pair<ll, ll> PL;
  vector<PL> dp(n + 1), ma_acc(n + 1);
  dp[0] = PL(0, 0);
  ma_acc[0] = PL(0, 0);
  REP(i, 0, n) {
    PL res(0,0);
    res = max(res, dp[i]);
    PL sub2 = ma_acc[get_pos(t[i] - k)];
    sub2.second += d[i];
    if (d[i] > hi) {
      // Yuki does this.
      sub2.first += 1;
    }
    res = max(res, sub2);
    dp[i + 1] = res;
    ma_acc[i + 1] = max(ma_acc[i], res);
  }
  ll tot = -dp[n].second;
  REP(i, 0, n) {
    tot += d[i];
  }
  cout << tot << endl;
}
