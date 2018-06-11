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

const ll inf = 1e12;

const int N = 100100;
ll x[N], v[N];

ll trunc_div(ll x, ll y) {
  if (x >= 0) return x / y;
  return -((-x + y - 1) / y);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll l, wmax;
  cin >> n >> l >> wmax;
  REP(i, 0, n) cin >> x[i] >> v[i];
  VL pos, neg;
  REP(i, 0, n) {
    if (v[i] == 1) {
      pos.push_back(x[i]);
    } else {
      neg.push_back(x[i]);
    }
  }
  sort(neg.begin(), neg.end());
  ll tot = 0;
  for (ll p: pos) {
    ll lim = p + l;
    if (wmax == 1) {
      if (p >= 0) lim = inf;
    } else {
      lim = max(lim, trunc_div((1 + wmax) * p, (wmax - 1)) + 1 - l);
    }
    lim = max(lim, trunc_div((wmax - 1) * p, wmax + 1) + 1 - l);
    if (0) {
      DEBUGP(p);
      DEBUGP(lim);
    }
    tot += neg.end() - lower_bound(neg.begin(), neg.end(), lim);
  }
  cout << tot << "\n";
}
