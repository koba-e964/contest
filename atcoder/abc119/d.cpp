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




int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int a, b, q;
  cin >> a >> b >> q;
  VL s(a + 2), t(b + 2), x(q);
  REP(i, 0, a) cin >> s[i + 1];
  REP(i, 0, b) cin >> t[i + 1];
  REP(i, 0, q) cin >> x[i];
  const ll inf = 1e16;
  s[0] = t[0] = -inf;
  s[a + 1] = t[b + 1] = inf;
  REP(i, 0, q) {
    ll k = x[i];
    int sa = lower_bound(s.begin(), s.end(), k) - s.begin() - 1;
    int sb = sa + 1;
    int ta = lower_bound(t.begin(), t.end(), k) - t.begin() - 1;
    int tb = ta + 1;
    ll mi = 1e18;
    mi = min(mi, max(k - s[sa], k - t[ta]));
    mi = min(mi, max(-k + s[sb], -k + t[tb]));
    mi = min(mi, (k - s[sa]) + (t[tb] - k) + min(k - s[sa], t[tb] - k));
    mi = min(mi, (k - t[ta]) + (s[sb] - k) + min(k - t[ta], s[sb] - k));
  cout << mi << endl;
  }
}
