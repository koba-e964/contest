#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>
#include <set>
#include <map>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const ll inf = 2.1e18;

void add(map<ll, int> &m, ll x) {
  m[x]++;
}
void del(map<ll, int> &m, ll x) {
  if (m[x] == 1) {
    m.erase(x);
  } else {
    m[x]--;
  }
}


int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  map<ll, int> pre, post;
  REP(i, 1, n) {
    add(post, a[i]);
  }
  ll ans = -inf;
  REP(i, 0, n) {
    if (i > 0) {
      add(pre, a[i - 1]);
      del(post, a[i]);
    }
    ll ma = -inf;
    auto it1 = post.lower_bound(a[i] + 1);
    if (it1 == post.end()) continue;
    auto it2 = pre.upper_bound(a[i] - 1);
    if (it2 == pre.begin()) continue;
    it2--;
    ll val2 = it2->first;
    ll val1 = it1->first;
    ma = max(ma, val2 * val1 * a[i]);
    ma = max(ma, val2 * post.rbegin()->first * a[i]);
    ma = max(ma, pre.begin()->first * val1 * a[i]);
    ma = max(ma, pre.begin()->first * post.rbegin()->first * a[i]);
    ans = max(ans, ma);
  }
  if (ans <= -inf) {
    cout << -1 << endl;
  } else {
    cout << ans << endl;
  }
}
