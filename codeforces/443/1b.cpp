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
typedef pair<int, ll> PIL;
const ll mod = 1e9 + 7;

ll k;

void push(vector<PIL> &target, const VI &a) {
  int n = a.size();
  REP(i, 0, n) {
    if (target.size() == 0 || target.back().first != a[i]) {
      target.push_back(PIL(a[i], 1));
    } else {
      target.back().second++;
      if (target.back().second >= k) {
	target.pop_back();
      }
    }
  }
}

vector<PIL> mul(const vector<PIL> &s, const vector<PIL> &t) {
  vector<PIL> ret(s);
  REP(i, 0, t.size()) {
    int kind = t[i].first;
    ll num = t[i].second;
    if (ret.size() == 0 || ret.back().first != kind) {
      ret.push_back(PIL(kind, num));
    } else {
      ret.back().second += num;
      ret.back().second %= k;
      if (ret.back().second == 0) {
	ret.pop_back();
      }
    }
  }
  return ret;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll m;
  cin >> n >> k >> m;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  vector<PIL> one;
  push(one, a);
  if (m == 1) {
    ll ans = 0;
    for (PIL x: one) ans += x.second;
    cout << ans << "\n";
    return 0;
  }
  vector<PIL> two(one);
  push(two, a);
  if (one.size() < two.size()) {
    ll s = 0, t = 0;
    for (PIL x: one) s += x.second;
    for (PIL x: two) t += x.second;
    cout << s + (t - s) * (m - 1) << "\n";
    return 0;
  }
  ll e = m;
  vector<PIL> sum;
  vector<PIL> cur(one);
  while (e > 0) {
    if (e % 2) {
      sum = mul(sum, cur);
    }
    cur = mul(cur, cur);
    e /= 2;
  }
  ll ans = 0;
  for (PIL x: sum) ans += x.second;
  cout << ans << "\n";
}
